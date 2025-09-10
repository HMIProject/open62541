#![cfg_attr(
    not(feature = "tokio"),
    expect(
        dead_code,
        reason = "Some methods are only used when this feature is enabled."
    )
)]
#![cfg_attr(
    not(feature = "tokio"),
    expect(
        unreachable_pub,
        reason = "Some types/methods only need to be public when this features is enabled."
    )
)]

pub(crate) mod delete_monitored_items;

use std::marker::PhantomData;

use crate::{attributes, ua, Attribute, DataValue};

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
