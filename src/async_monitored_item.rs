mod create_monitored_items;

use std::{
    marker::PhantomData,
    pin::Pin,
    sync::{Arc, Weak},
    task::{self, Poll},
    time::Duration,
};

use futures_core::Stream;
use tokio::sync::mpsc;

use crate::{
    attributes,
    monitored_item::{DataChange, Unknown},
    ua, AsyncSubscription, DataType as _, Error, MonitoredItemAttribute, MonitoredItemKind,
    MonitoredItemValue, MonitoringFilter, Result,
};

#[derive(Debug)]
pub struct MonitoredItemBuilder<K: MonitoredItemKind> {
    node_ids: Vec<ua::NodeId>,
    attribute_id: ua::AttributeId,
    monitoring_mode: Option<ua::MonitoringMode>,
    #[expect(clippy::option_option, reason = "implied default vs. unset")]
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
    /// By default, monitored items emit [`DataValue`](crate::DataValue) of the appropriate subtype matching the given
    /// attribute. If the attribute is set to [`ua::AttributeId::EVENTNOTIFIER_T`], they emit
    /// `ua::Array<ua::Variant>` instead.
    ///
    /// Default value is [`ua::AttributeId::VALUE_T`].
    ///
    /// See [`Self::attribute_id()`] to set the attribute ID at runtime.
    ///
    /// # Examples
    ///
    /// ```
    /// use open62541::{DataValue, MonitoredItemBuilder, MonitoredItemValue, ua};
    /// use open62541_sys::UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME;
    ///
    /// # async fn wrap(subscription: open62541::AsyncSubscription) -> open62541::Result<()> {
    /// let node_ids = [ua::NodeId::ns0(UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME)];
    ///
    /// let mut results = MonitoredItemBuilder::new(node_ids)
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
    /// use open62541::{DataValue, MonitoredItemBuilder, MonitoredItemValue, ua};
    /// use open62541_sys::UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME;
    ///
    /// # async fn wrap(subscription: open62541::AsyncSubscription) -> open62541::Result<()> {
    /// let node_ids = [ua::NodeId::ns0(UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME)];
    /// let attribute_id = ua::AttributeId::BROWSENAME;
    ///
    /// let mut results = MonitoredItemBuilder::new(node_ids)
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
            crate::delete_monitored_items::call(client, &request);

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
    #[deprecated = "AsyncMonitoredItem implements Stream."]
    pub fn into_stream(self) -> impl Stream<Item = K::Value> + Send + Sync + 'static {
        self
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

        crate::delete_monitored_items::call(&client, &request);
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
