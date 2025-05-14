mod create_monitored_items;
mod delete_monitored_items;

use std::{
    marker::PhantomData,
    pin::Pin,
    sync::{Arc, Weak},
    task::{self, Poll},
    time::Duration,
};

use futures_core::Stream;
use futures_util::stream;
use tokio::sync::mpsc;

use crate::{attributes, ua, AsyncSubscription, DataType as _, Error, MonitoringFilter, Result};

use self::sealed::{DataChange, MonitoredItemAttribute, MonitoredItemKind, Unknown};

#[derive(Debug)]
pub struct MonitoredItemBuilder<K: MonitoredItemKind> {
    node_ids: Vec<ua::NodeId>,
    attribute_id: ua::AttributeId,
    monitoring_mode: Option<ua::MonitoringMode>,
    #[allow(clippy::option_option)]
    sampling_interval: Option<Option<Duration>>,
    filter: Option<Box<dyn MonitoringFilter>>,
    queue_size: Option<u32>,
    discard_oldest: Option<bool>,
    _kind: PhantomData<K>,
}

impl MonitoredItemBuilder<DataChange<attributes::Value>> {
    pub fn new(node_ids: impl IntoIterator<Item = ua::NodeId>) -> Self {
        Self {
            node_ids: node_ids.into_iter().collect(),
            // Use explicit default to uphold invariant of typestate.
            attribute_id: ua::AttributeId::VALUE,
            monitoring_mode: None,
            sampling_interval: None,
            filter: None,
            queue_size: None,
            discard_oldest: None,
            _kind: PhantomData,
        }
    }
}

// Note: The default values in the docs below come from `UA_MonitoredItemCreateRequest_default()`.
impl<K: MonitoredItemKind> MonitoredItemBuilder<K> {
    /// Sets attribute.
    ///
    /// By default, monitored items emit [`MonitoredItemValue::DataChange`]. If the attribute is set
    /// to [`attributes::EventNotifier`], they emit [`MonitoredItemValue::Event`] instead.
    ///
    /// Default value is [`attributes::Value`].
    ///
    /// See [`Self::attribute_id()`] to set the attribute ID at runtime.
    #[must_use]
    pub fn attribute<T: MonitoredItemAttribute>(
        self,
        attribute: T,
    ) -> MonitoredItemBuilder<T::Kind> {
        let Self {
            node_ids,
            attribute_id: _,
            monitoring_mode,
            sampling_interval,
            filter,
            queue_size,
            discard_oldest,
            _kind,
        } = self;

        MonitoredItemBuilder {
            node_ids,
            attribute_id: attribute.id(),
            monitoring_mode,
            sampling_interval,
            filter,
            queue_size,
            discard_oldest,
            _kind: PhantomData,
        }
    }

    /// Sets attribute ID.
    ///
    /// By default, monitored items emit [`MonitoredItemValue::DataChange`]. If the attribute is set
    /// to [`ua::AttributeId::EVENTNOTIFIER`], they emit [`MonitoredItemValue::Event`] instead.
    ///
    /// Default value is [`ua::AttributeId::VALUE`].
    ///
    /// See [`ua::MonitoredItemCreateRequest::with_attribute_id()`].
    #[must_use]
    pub fn attribute_id(self, attribute_id: ua::AttributeId) -> MonitoredItemBuilder<Unknown> {
        let Self {
            node_ids,
            attribute_id: _,
            monitoring_mode,
            sampling_interval,
            filter,
            queue_size,
            discard_oldest,
            _kind,
        } = self;

        MonitoredItemBuilder {
            node_ids,
            attribute_id,
            monitoring_mode,
            sampling_interval,
            filter,
            queue_size,
            discard_oldest,
            _kind: PhantomData,
        }
    }
}

// Note: The default values in the docs below come from `UA_MonitoredItemCreateRequest_default()`.
impl<K: MonitoredItemKind> MonitoredItemBuilder<K> {
    /// Sets monitoring mode.
    ///
    /// Default value is [`ua::MonitoringMode::REPORTING`].
    ///
    /// See [`ua::MonitoredItemCreateRequest::with_monitoring_mode()`].
    #[must_use]
    pub fn monitoring_mode(mut self, monitoring_mode: ua::MonitoringMode) -> Self {
        self.monitoring_mode = Some(monitoring_mode);
        self
    }

    /// Sets sampling interval.
    ///
    /// Default value is 250.0 ms.
    ///
    /// See [`ua::MonitoringParameters::with_sampling_interval()`].
    #[must_use]
    pub const fn sampling_interval(mut self, sampling_interval: Option<Duration>) -> Self {
        self.sampling_interval = Some(sampling_interval);
        self
    }

    /// Sets filter.
    ///
    /// Default value is no filter.
    ///
    /// See [`ua::MonitoringParameters::with_filter()`].
    #[must_use]
    pub fn filter(mut self, filter: impl MonitoringFilter) -> Self {
        self.filter = Some(Box::new(filter));
        self
    }

    /// Sets requested size of the monitored item queue.
    ///
    /// Default value is 1.
    ///
    /// See [`ua::MonitoringParameters::with_queue_size()`].
    #[must_use]
    pub const fn queue_size(mut self, queue_size: u32) -> Self {
        self.queue_size = Some(queue_size);
        self
    }

    /// Sets discard policy.
    ///
    /// Default value is `true`.
    ///
    /// See [`ua::MonitoringParameters::with_discard_oldest()`].
    #[must_use]
    pub const fn discard_oldest(mut self, discard_oldest: bool) -> Self {
        self.discard_oldest = Some(discard_oldest);
        self
    }

    /// Creates monitored items.
    ///
    /// This creates one or more new monitored items. Returns one result for each node ID.
    ///
    /// # Errors
    ///
    /// This fails when the entire request is not successful. Errors for individual node IDs are
    /// returned as error elements inside the resulting list.
    pub async fn create(
        self,
        subscription: &AsyncSubscription,
    ) -> Result<Vec<Result<(ua::MonitoredItemCreateResult, AsyncMonitoredItem<K>)>>> {
        let Some(client) = &subscription.client().upgrade() else {
            return Err(Error::internal("client should not be dropped"));
        };
        let subscription_id = subscription.subscription_id();

        let request = self.into_request(subscription_id);
        let result_count = request.items_to_create().map_or(0, <[_]>::len);
        let (response, rxs) = create_monitored_items::call(client, &request).await?;

        let Some(mut results) = response.into_results() else {
            return Err(Error::internal("expected monitoring item results"));
        };

        if results.len() != result_count || rxs.len() != result_count {
            // This should not happen. In any case, we cannot associate returned items with their
            // incoming node IDs. Clean up the items that we received to not leave them dangling.
            //
            let monitored_item_ids = results
                .iter()
                .filter(|result| result.status_code().is_good())
                .map(ua::MonitoredItemCreateResult::monitored_item_id)
                .collect::<Vec<_>>();
            let request = ua::DeleteMonitoredItemsRequest::init()
                .with_subscription_id(subscription_id)
                .with_monitored_item_ids(&monitored_item_ids);
            // This request is processed asynchronously. Errors are logged asynchronously too.
            delete_monitored_items::call(client, &request);

            return Err(Error::internal("unexpected number of monitored items"));
        }

        let results = results
            .drain_all()
            .zip(rxs)
            .map(|(result, rx)| {
                Error::verify_good(&result.status_code())?;

                let monitored_item = AsyncMonitoredItem::new(
                    client,
                    subscription_id,
                    result.monitored_item_id(),
                    rx,
                );

                Ok((result, monitored_item))
            })
            .collect();

        Ok(results)
    }

    fn into_request(self, subscription_id: ua::SubscriptionId) -> ua::CreateMonitoredItemsRequest {
        let Self {
            node_ids,
            attribute_id,
            monitoring_mode,
            sampling_interval,
            filter,
            queue_size,
            discard_oldest,
            _kind: _,
        } = self;

        let items_to_create = node_ids
            .into_iter()
            .map(|node_id| {
                let mut request = ua::MonitoredItemCreateRequest::default()
                    .with_node_id(&node_id)
                    .with_attribute_id(&attribute_id);

                if let Some(monitoring_mode) = monitoring_mode.as_ref() {
                    request = request.with_monitoring_mode(monitoring_mode);
                }
                if let Some(&sampling_interval) = sampling_interval.as_ref() {
                    request = request.with_sampling_interval(sampling_interval);
                }
                if let Some(filter) = filter.as_ref() {
                    request = request.with_filter(filter);
                }
                if let Some(&queue_size) = queue_size.as_ref() {
                    request = request.with_queue_size(queue_size);
                }
                if let Some(&discard_oldest) = discard_oldest.as_ref() {
                    request = request.with_discard_oldest(discard_oldest);
                }

                request
            })
            .collect::<Vec<_>>();

        ua::CreateMonitoredItemsRequest::init()
            .with_subscription_id(subscription_id)
            .with_items_to_create(&items_to_create)
    }
}

/// Value emitted from monitored item notification.
///
/// The variant depends on the attribute ID passed to [`MonitoredItemBuilder::attribute_id()`].
#[derive(Debug)]
pub enum MonitoredItemValue {
    /// Data change payload.
    ///
    /// This is emitted for attribute IDs other than [`ua::AttributeId::EVENTNOTIFIER`].
    DataChange { value: ua::DataValue },

    /// Event payload.
    ///
    /// This is emitted for attribute ID [`ua::AttributeId::EVENTNOTIFIER`].
    Event { fields: ua::Array<ua::Variant> },
}

impl MonitoredItemValue {
    /// Shortcut for accessing data change value.
    ///
    /// This returns `None` for [`MonitoredItemValue::Event`].
    #[must_use]
    pub fn value(&self) -> Option<&ua::Variant> {
        match self {
            MonitoredItemValue::DataChange { value } => value.value(),
            MonitoredItemValue::Event { fields: _ } => None,
        }
    }
}

/// Monitored item (with asynchronous API).
#[derive(Debug)]
pub struct AsyncMonitoredItem<K: MonitoredItemKind = DataChange<attributes::Value>> {
    client: Weak<ua::Client>,
    subscription_id: ua::SubscriptionId,
    monitored_item_id: ua::MonitoredItemId,
    rx: mpsc::Receiver<MonitoredItemValue>,
    _kind: PhantomData<K>,
}

impl<K: MonitoredItemKind> AsyncMonitoredItem<K> {
    fn new(
        client: &Arc<ua::Client>,
        subscription_id: ua::SubscriptionId,
        monitored_item_id: ua::MonitoredItemId,
        rx: mpsc::Receiver<MonitoredItemValue>,
    ) -> Self {
        Self {
            client: Arc::downgrade(client),
            subscription_id,
            monitored_item_id,
            rx,
            _kind: PhantomData,
        }
    }

    /// Waits for next value from server.
    ///
    /// This waits for the next value received for this monitored item. Returns `None` when item has
    /// been closed and no more updates will be received.
    pub async fn next(&mut self) -> Option<K::Value> {
        // This mirrors `<Self as Stream>::poll_next()` but does not require `self` to be pinned.
        self.rx.recv().await.map(K::map_value)
    }

    /// Turns monitored item into stream.
    ///
    /// The stream will emit all value updates as they are being received. If the client disconnects
    /// or the corresponding subscription is deleted, the stream is closed.
    //
    // TODO: Remove this? Consuming `AsyncMonitoredItem` to turn it into a stream drops it, removing
    // the monitored item subscription immediately. See `Drop` implementation below.
    pub fn into_stream(self) -> impl Stream<Item = K::Value> + Send + Sync + 'static {
        stream::unfold(self, move |mut this| async move {
            this.next().await.map(|value| (value, this))
        })
    }
}

impl<K: MonitoredItemKind> Drop for AsyncMonitoredItem<K> {
    fn drop(&mut self) {
        let Some(client) = self.client.upgrade() else {
            return;
        };

        let request = ua::DeleteMonitoredItemsRequest::init()
            .with_subscription_id(self.subscription_id)
            .with_monitored_item_ids(&[self.monitored_item_id]);

        delete_monitored_items::call(&client, &request);
    }
}

impl<K: MonitoredItemKind> Stream for AsyncMonitoredItem<K> {
    type Item = K::Value;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Option<Self::Item>> {
        // This mirrors `AsyncMonitoredItem::next()` and implements the `Stream` trait.
        self.as_mut()
            .rx
            .poll_recv(cx)
            .map(|value| value.map(K::map_value))
    }
}

impl<K: MonitoredItemKind> Unpin for AsyncMonitoredItem<K> {}

mod sealed {
    use std::marker::PhantomData;

    use crate::Attribute;

    /// Typestate used in [`super::MonitoredItemBuilder`].
    pub trait MonitoredItemKind: Send + Sync + 'static {
        type Value: Send;

        fn map_value(value: super::MonitoredItemValue) -> Self::Value;
    }

    /// Typestate for [`MonitoredItemKind`] that yields data change notifications.
    #[derive(Debug)]
    pub struct DataChange<T: Attribute>(PhantomData<T>);

    impl<T: DataChangeAttribute + Send + Sync + 'static> MonitoredItemKind for DataChange<T> {
        // TODO: Use more specific type.
        type Value = super::MonitoredItemValue;

        fn map_value(value: super::MonitoredItemValue) -> Self::Value {
            value
        }
    }

    /// Typestate for [`MonitoredItemKind`] that yields event notifications.
    #[derive(Debug)]
    pub struct Event;

    impl MonitoredItemKind for Event {
        // TODO: Use more specific type.
        type Value = super::MonitoredItemValue;

        fn map_value(value: super::MonitoredItemValue) -> Self::Value {
            value
        }
    }

    /// Typestate for [`MonitoredItemKind`] that yields notifications.
    ///
    /// This is used for untyped or mixed-type notifications.
    #[derive(Debug)]
    pub struct Unknown;

    impl MonitoredItemKind for Unknown {
        type Value = super::MonitoredItemValue;

        fn map_value(value: super::MonitoredItemValue) -> Self::Value {
            value
        }
    }

    /// Attribute that yields data change notifications.
    ///
    /// This is implemented for all attributes except [`crate::attributes::EventNotifier`].
    pub trait DataChangeAttribute: Attribute {}

    /// Helper trait to get correct [`MonitoredItemKind`].
    ///
    /// Given an arbitrary attribute, including [`crate::attributes::EventNotifier`], this helps get
    /// the right [`MonitoredItemKind`] implementation for [`super::MonitoredItemBuilder`].
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

    data_change_impl!(
        NodeId,
        NodeClass,
        BrowseName,
        DisplayName,
        Description,
        WriteMask,
        IsAbstract,
        Symmetric,
        InverseName,
        ContainsNoLoops,
        // We to _not_ implement `DataChange` kind for `EventNotifier`, because the attribute uses a
        // dedicated callback function yielding `ua::Array<ua::Variant>` instead of `ua::DataValue`.
        Value,
        DataType,
        ValueRank,
        ArrayDimensions,
        AccessLevel,
        AccessLevelEx,
        MinimumSamplingInterval,
        Historizing,
        Executable,
    );

    impl MonitoredItemAttribute for crate::attributes::EventNotifier {
        type Kind = Event;
    }
}
