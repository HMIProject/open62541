use std::{marker::PhantomData, pin::Pin, task, time::Duration};

use futures_core::Stream;
use tokio::sync::mpsc::{self, error::TrySendError};

use crate::{
    attributes, create_monitored_items_callback,
    monitored_item::{DataChange, Unknown},
    ua, AsyncSubscription, Error, MonitoredItemAttribute, MonitoredItemCreateRequestBuilder,
    MonitoredItemHandle, MonitoredItemKind, MonitoredItemValue, MonitoringFilter, Result,
};

/// Maximum number of buffered values.
// TODO: Think about appropriate buffer size or let the caller decide.
const DEFAULT_STREAM_BUFFER_SIZE: usize = 3;

#[derive(Debug)]
pub struct AsyncMonitoredItemBuilder<K: MonitoredItemKind> {
    create_request: MonitoredItemCreateRequestBuilder<K>,
}

impl AsyncMonitoredItemBuilder<DataChange<attributes::Value>> {
    pub fn new(node_ids: impl IntoIterator<Item = ua::NodeId>) -> Self {
        Self {
            create_request: MonitoredItemCreateRequestBuilder::new(node_ids),
        }
    }
}

// Note: The default values in the docs below come from `UA_MonitoredItemCreateRequest_default()`.
impl<K: MonitoredItemKind> AsyncMonitoredItemBuilder<K> {
    /// Sets attribute.
    ///
    /// By default, monitored items emit [`DataValue`](crate::DataValue) of the appropriate subtype
    /// matching the given attribute. If the attribute is set to [`ua::AttributeId::EVENTNOTIFIER_T`],
    /// they emit `ua::Array<ua::Variant>` instead.
    ///
    /// Default value is [`ua::AttributeId::VALUE_T`].
    ///
    /// See [`Self::attribute_id()`] to set the attribute ID at runtime.
    ///
    /// # Examples
    ///
    /// ```
    /// use open62541::{DataValue, AsyncMonitoredItemBuilder, MonitoredItemValue, ua};
    /// use open62541_sys::UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME;
    ///
    /// # async fn wrap(subscription: open62541::AsyncSubscription) -> open62541::Result<()> {
    /// let node_ids = [ua::NodeId::ns0(UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME)];
    ///
    /// let mut results = AsyncMonitoredItemBuilder::new(node_ids)
    ///     .attribute(ua::AttributeId::BROWSENAME_T)
    ///     .create(&subscription)
    ///     .await?;
    /// let (_, mut monitored_item) = results.pop().unwrap()?;
    ///
    /// if let Some(value) = monitored_item.next().await {
    ///     // Typed value for attribute `BROWSENAME` above.
    ///     let value: DataValue<ua::QualifiedName> = value;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn attribute<T: MonitoredItemAttribute>(
        self,
        attribute: T,
    ) -> AsyncMonitoredItemBuilder<T::Kind> {
        let Self { create_request } = self;
        AsyncMonitoredItemBuilder {
            create_request: create_request.attribute(attribute),
        }
    }

    /// Sets attribute ID.
    ///
    /// When using this method, monitored items emit [`MonitoredItemValue`] instead of the specific
    /// type. See [`Self::attribute()`] for a type-safe alternative that yields appropriately typed
    /// values for the given monitored attribute directly.
    ///
    /// Default value is [`ua::AttributeId::VALUE`].
    ///
    /// See [`ua::MonitoredItemCreateRequest::with_attribute_id()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use open62541::{DataValue, AsyncMonitoredItemBuilder, MonitoredItemValue, ua};
    /// use open62541_sys::UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME;
    ///
    /// # async fn wrap(subscription: open62541::AsyncSubscription) -> open62541::Result<()> {
    /// let node_ids = [ua::NodeId::ns0(UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME)];
    /// let attribute_id = ua::AttributeId::BROWSENAME;
    ///
    /// let mut results = AsyncMonitoredItemBuilder::new(node_ids)
    ///     .attribute_id(attribute_id)
    ///     .create(&subscription)
    ///     .await?;
    /// let (_, mut monitored_item) = results.pop().unwrap()?;
    ///
    /// if let Some(value) = monitored_item.next().await {
    ///     // Dynamically typed value for any attribute.
    ///     let value: MonitoredItemValue = value;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn attribute_id(self, attribute_id: ua::AttributeId) -> AsyncMonitoredItemBuilder<Unknown> {
        let Self { create_request } = self;
        AsyncMonitoredItemBuilder {
            create_request: create_request.attribute_id(attribute_id),
        }
    }
}

// Note: The default values in the docs below come from `UA_MonitoredItemCreateRequest_default()`.
impl<K: MonitoredItemKind> AsyncMonitoredItemBuilder<K> {
    /// Sets monitoring mode.
    ///
    /// Default value is [`ua::MonitoringMode::REPORTING`].
    ///
    /// See [`ua::MonitoredItemCreateRequest::with_monitoring_mode()`].
    #[must_use]
    pub fn monitoring_mode(mut self, monitoring_mode: ua::MonitoringMode) -> Self {
        self.create_request = self.create_request.monitoring_mode(monitoring_mode);
        self
    }

    /// Sets sampling interval.
    ///
    /// Default value is 250.0 ms.
    ///
    /// See [`ua::MonitoringParameters::with_sampling_interval()`].
    #[must_use]
    pub fn sampling_interval(mut self, sampling_interval: Option<Duration>) -> Self {
        self.create_request = self.create_request.sampling_interval(sampling_interval);
        self
    }

    /// Sets filter.
    ///
    /// Default value is no filter.
    ///
    /// See [`ua::MonitoringParameters::with_filter()`].
    #[must_use]
    pub fn filter(mut self, filter: impl MonitoringFilter) -> Self {
        self.create_request = self.create_request.filter(filter);
        self
    }

    /// Sets requested size of the monitored item queue.
    ///
    /// Default value is 1.
    ///
    /// See [`ua::MonitoringParameters::with_queue_size()`].
    #[must_use]
    pub fn queue_size(mut self, queue_size: u32) -> Self {
        self.create_request = self.create_request.queue_size(queue_size);
        self
    }

    /// Sets discard policy.
    ///
    /// Default value is `true`.
    ///
    /// See [`ua::MonitoringParameters::with_discard_oldest()`].
    #[must_use]
    pub fn discard_oldest(mut self, discard_oldest: bool) -> Self {
        self.create_request = self.create_request.discard_oldest(discard_oldest);
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

        let Self {
            create_request: create_request_builder,
        } = self;

        let mut rxs = Vec::with_capacity(create_request_builder.node_ids().len());
        let create_value_callback_fn = |index: usize| {
            let (tx, rx) = AsyncMonitoredItem::<K>::channel(DEFAULT_STREAM_BUFFER_SIZE);
            rxs.push(rx);
            debug_assert_eq!(index, rxs.len());
            move |monitored_item_value| {
                if let Err(err) = tx.try_send(monitored_item_value) {
                    match err {
                        TrySendError::Full(_value) => {
                            // We cannot blockingly wait, because that would block `UA_Client_run_iterate()`
                            // in our event loop, potentially preventing the receiver from clearing the stream.
                            // The monitored value might contain sensitive information and must not be logged!
                            log::error!("Discarding monitored item value: stream buffer (size = {buffer_size}) is full", buffer_size = tx.capacity());
                        }
                        TrySendError::Closed(_) => {
                            // Received has disappeared and the value is no longer needed.
                        }
                    }
                }
            }
        };

        let results = create_monitored_items_callback(
            client,
            subscription_id,
            create_request_builder,
            create_value_callback_fn,
        )
        .await?;
        debug_assert_eq!(results.len(), rxs.len());

        let results = results
            .into_iter()
            .zip(rxs)
            .map(|(result, rx)| {
                let (result, handle) = result?;
                let monitored_item = AsyncMonitoredItem::new(handle, rx);
                Ok((result, monitored_item))
            })
            .collect();

        Ok(results)
    }
}

/// Monitored item (with asynchronous API).
#[derive(Debug)]
pub struct AsyncMonitoredItem<K: MonitoredItemKind = DataChange<attributes::Value>> {
    #[expect(dead_code, reason = "The handle keeps the monitored item alive.")]
    handle: MonitoredItemHandle,
    rx: mpsc::Receiver<MonitoredItemValue>,
    _kind: PhantomData<K>,
}

impl<K: MonitoredItemKind> AsyncMonitoredItem<K> {
    pub(crate) fn channel(
        buffer_size: usize,
    ) -> (
        mpsc::Sender<MonitoredItemValue>,
        mpsc::Receiver<MonitoredItemValue>,
    ) {
        mpsc::channel(buffer_size)
    }

    pub(crate) const fn new(
        handle: MonitoredItemHandle,
        rx: mpsc::Receiver<MonitoredItemValue>,
    ) -> Self {
        Self {
            handle,
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
    #[deprecated = "AsyncMonitoredItem implements Stream."]
    pub fn into_stream(self) -> impl Stream<Item = K::Value> + Send + Sync + 'static {
        self
    }
}

impl<K: MonitoredItemKind> Stream for AsyncMonitoredItem<K> {
    type Item = K::Value;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Option<Self::Item>> {
        // This mirrors `AsyncMonitoredItem::next()` and implements the `Stream` trait.
        self.as_mut()
            .rx
            .poll_recv(cx)
            .map(|value| value.map(K::map_value))
    }
}

impl<K: MonitoredItemKind> Unpin for AsyncMonitoredItem<K> {}
