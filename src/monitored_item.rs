#![cfg_attr(
    not(feature = "tokio"),
    expect(
        dead_code,
        reason = "Some methods are only used when this feature is enabled."
    )
)]

mod create_request_builder;
pub use self::create_request_builder::MonitoredItemCreateRequestBuilder;

// TODO: Remove pub(crate).
pub(crate) mod delete_monitored_items;

use std::{
    marker::PhantomData,
    sync::{Arc, Weak},
};

use crate::{attributes, ua, Attribute, DataType as _, DataValue, Error, Result};

/// Handle for a single monitored item.
///
/// Keeps the monitored item at the server alive until either deleted or dropped.
#[derive(Debug)]
pub(crate) struct MonitoredItemHandle {
    client: Weak<ua::Client>,
    subscription_id: ua::SubscriptionId,
    monitored_item_id: Option<ua::MonitoredItemId>,
}

impl MonitoredItemHandle {
    pub(crate) fn new(
        client: &Arc<ua::Client>,
        subscription_id: ua::SubscriptionId,
        monitored_item_id: ua::MonitoredItemId,
    ) -> Self {
        Self {
            client: Arc::downgrade(client),
            subscription_id,
            monitored_item_id: Some(monitored_item_id),
        }
    }

    fn before_delete(&mut self) -> Result<(ua::DeleteMonitoredItemsRequest, ua::MonitoredItemId)> {
        let Some(monitored_item_id) = self.monitored_item_id.take() else {
            return Err(Error::internal("already deleted"));
        };
        let request = ua::DeleteMonitoredItemsRequest::init()
            .with_subscription_id(self.subscription_id)
            .with_monitored_item_ids(&[monitored_item_id]);

        Ok((request, monitored_item_id))
    }

    /// Reverts the changes of [`before_delete()`](Self::before_delete) for retrying.
    ///
    /// This is unlikely to happen.
    fn after_delete_failed(&mut self, monitored_item_id: ua::MonitoredItemId) {
        debug_assert!(self.monitored_item_id.is_none());
        // Put the id back for a retry.
        self.monitored_item_id = Some(monitored_item_id);
    }

    /// Deletes the monitored item at the server.
    ///
    /// No new notifications will be received after the invocation succeeded.
    ///
    /// This method should only be called once. After it succeeded
    /// any subsequent invocation will fail with an internal error.
    ///
    /// # Errors
    ///
    /// This fails when the monitored item has already been deleted before,
    /// when connection is interrupted, or when the server returns an error.
    pub(crate) async fn delete(&mut self) -> Result<ua::DeleteMonitoredItemsResponse> {
        // Consume the `Option` field first to ensure that this method
        // could never be called twice.
        let (request, monitored_item_id) = self.before_delete()?;

        let Some(client) = self.client.upgrade() else {
            // No rollback, because the client is gone forever.
            return Err(Error::internal("no client"));
        };

        delete_monitored_items::call(&client, &request)
            .await
            .inspect_err(|_| {
                // Rollback, i.e. revert changes on invocation error.
                self.after_delete_failed(monitored_item_id);
            })
    }
}

impl Drop for MonitoredItemHandle {
    fn drop(&mut self) {
        let Ok((request, _monitored_item_id)) = self.before_delete() else {
            // Already deleted before.
            return;
        };

        let Some(client) = self.client.upgrade() else {
            log::debug!("Cannot delete monitored_item {request:?} on drop without client");
            return;
        };

        // Response errors will only be logged.
        if let Err(err) = delete_monitored_items::send_request(&client, &request) {
            log::warn!(
                "Failed to sent request for deleting monitored item {request:?} on drop: {err:#}"
            );
        }
    }
}

/// Value emitted from monitored item notification.
///
/// This depends on the attribute ID passed to [`MonitoredItemBuilder::attribute_id()`](crate::MonitoredItemBuilder::attribute_id).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonitoredItemValue(MonitoredItemValueInner);

impl MonitoredItemValue {
    #[must_use]
    pub(crate) const fn data_change(value: ua::DataValue) -> Self {
        Self(MonitoredItemValueInner::DataChange { value })
    }

    #[must_use]
    pub(crate) const fn event(fields: ua::Array<ua::Variant>) -> Self {
        Self(MonitoredItemValueInner::Event { fields })
    }

    /// Gets data change payload.
    ///
    /// This returns `None` for event monitored items.
    #[must_use]
    pub const fn value(&self) -> Option<&ua::DataValue> {
        match &self.0 {
            MonitoredItemValueInner::DataChange { value } => Some(value),
            MonitoredItemValueInner::Event { fields: _ } => None,
        }
    }

    /// Gets event payload.
    ///
    /// This returns `None` for data change monitored items.
    #[must_use]
    pub const fn fields(&self) -> Option<&[ua::Variant]> {
        match &self.0 {
            MonitoredItemValueInner::DataChange { value: _ } => None,
            MonitoredItemValueInner::Event { fields } => Some(fields.as_slice()),
        }
    }

    #[must_use]
    fn into_inner(self) -> MonitoredItemValueInner {
        self.0
    }
}

// We consider both variants as distinct by deriving `Eq`.
// But there is no canonical ordering between those variants
// for implementing `Ord`.
#[derive(Debug, Clone, PartialEq, Eq)]
enum MonitoredItemValueInner {
    /// Data change payload.
    ///
    /// This is emitted for attribute IDs other than [`ua::AttributeId::EVENTNOTIFIER`].
    DataChange { value: ua::DataValue },

    /// Event payload.
    ///
    /// This is emitted for attribute ID [`ua::AttributeId::EVENTNOTIFIER`].
    Event { fields: ua::Array<ua::Variant> },
}

/// Sealed typestate trait.
pub trait MonitoredItemKind: sealed::MonitoredItemKind + Send + Sync + 'static {
    type Value;

    fn map_value(value: MonitoredItemValue) -> Self::Value;
}

/// Typestate for [`MonitoredItemKind`] that yields data change notifications.
#[derive(Debug)]
pub struct DataChange<T: Attribute>(PhantomData<T>);

impl<T: DataChangeAttribute + Send + Sync + 'static> MonitoredItemKind for DataChange<T> {
    type Value = DataValue<T::Value>;

    fn map_value(value: MonitoredItemValue) -> Self::Value {
        match value.into_inner() {
            MonitoredItemValueInner::DataChange { value } => value.cast(),
            MonitoredItemValueInner::Event { fields: _ } => {
                // PANIC: Typestate uses attribute ID to enforce callback method.
                unreachable!("unexpected event payload in data change notification");
            }
        }
    }
}

/// Typestate for [`MonitoredItemKind`] that yields event notifications.
#[derive(Debug)]
pub struct Event;

impl MonitoredItemKind for Event {
    type Value = ua::Array<ua::Variant>;

    fn map_value(value: MonitoredItemValue) -> Self::Value {
        match value.into_inner() {
            MonitoredItemValueInner::DataChange { value: _ } => {
                // PANIC: Typestate uses attribute ID to enforce callback method.
                unreachable!("unexpected data change payload in event notification");
            }
            MonitoredItemValueInner::Event { fields } => fields,
        }
    }
}

/// Typestate for [`MonitoredItemKind`] that yields notifications.
///
/// This is used for runtime and/or mixed-type notifications.
#[derive(Debug)]
pub struct Unknown;

impl MonitoredItemKind for Unknown {
    type Value = MonitoredItemValue;

    fn map_value(value: MonitoredItemValue) -> Self::Value {
        value
    }
}

/// Attribute that yields data change notifications.
///
/// This is implemented for all attributes except [`ua::AttributeId::EVENTNOTIFIER_T`].
trait DataChangeAttribute: Attribute {}

/// Attribute for [`MonitoredItemBuilder::attribute()`](crate::MonitoredItemBuilder::attribute).
pub trait MonitoredItemAttribute: Attribute {
    /// Matching [`MonitoredItemKind`] implementation for attribute.
    type Kind: MonitoredItemKind;
}

macro_rules! data_change_impl {
    ($($name:ident),* $(,)?) => {
        $(
            impl DataChangeAttribute for $crate::attributes::$name {}

            impl MonitoredItemAttribute for $crate::attributes::$name {
                type Kind = DataChange<$crate::attributes::$name>;
            }
        )*
    };
}

// Note: Array values are not supported yet in their typed form: previously, any such attempt would
// fail, because converting to `DataValue` expects scalar values.
//
// To give us some time to think about the best, typed representation of such non-scalar values, we
// remove their `impl` for now. Access is still possible with the non-typed attribute methods.
data_change_impl!(
    NodeId,
    NodeClass,
    BrowseName,
    DisplayName,
    Description,
    WriteMask,
    UserWriteMask,
    IsAbstract,
    Symmetric,
    InverseName,
    ContainsNoLoops,
    // We to _not_ implement `DataChange` kind for `EventNotifier`, because the attribute uses a
    // dedicated callback function yielding `ua::Array<ua::Variant>` instead of `ua::DataValue`.
    Value,
    DataType,
    ValueRank,
    // ArrayDimensions,
    AccessLevel,
    UserAccessLevel,
    MinimumSamplingInterval,
    Historizing,
    Executable,
    UserExecutable,
    DataTypeDefinition,
    // RolePermissions,
    // UserRolePermissions,
    AccessRestrictions,
    AccessLevelEx,
);

impl MonitoredItemAttribute for attributes::EventNotifier {
    type Kind = Event;
}

mod sealed {
    use crate::Attribute;

    pub trait MonitoredItemKind {}

    impl<T: Attribute> MonitoredItemKind for super::DataChange<T> {}

    impl MonitoredItemKind for super::Event {}

    impl MonitoredItemKind for super::Unknown {}
}
