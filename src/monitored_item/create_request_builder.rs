use std::{marker::PhantomData, time::Duration};

use crate::{
    attributes, ua, DataType as _, MonitoredItemAttribute, MonitoredItemKind, MonitoringFilter,
};

use super::{DataChange, Unknown};

/// Type-safe builder for [`ua::CreateMonitoredItemsRequest`].
#[derive(Debug)]
pub struct MonitoredItemCreateRequestBuilder<K: MonitoredItemKind> {
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

impl MonitoredItemCreateRequestBuilder<DataChange<attributes::Value>> {
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
//
// All builder methods are deliberately non-const to avoid breaking changes in the future.
impl<K: MonitoredItemKind> MonitoredItemCreateRequestBuilder<K> {
    /// Sets attribute.
    ///
    /// By default, monitored items emit [`DataValue`](crate::DataValue) of the appropriate subtype matching the given
    /// attribute. If the attribute is set to [`ua::AttributeId::EVENTNOTIFIER_T`], they emit
    /// `ua::Array<ua::Variant>` instead.
    ///
    /// Default value is [`ua::AttributeId::VALUE_T`].
    ///
    /// See [`Self::attribute_id()`] to set the attribute ID at runtime.
    //
    // TODO: Add examples like forAsyncMonitoredItemBuilder.
    #[must_use]
    pub fn attribute<T: MonitoredItemAttribute>(
        self,
        attribute: T,
    ) -> MonitoredItemCreateRequestBuilder<T::Kind> {
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

        MonitoredItemCreateRequestBuilder {
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
    /// When using this method, monitored items emit [`MonitoredItemValue`](crate::MonitoredItemValue)
    /// instead of the specific type. See [`Self::attribute()`] for a type-safe alternative that yields
    /// appropriately typed values for the given monitored attribute directly.
    ///
    /// Default value is [`ua::AttributeId::VALUE`].
    ///
    /// See [`ua::MonitoredItemCreateRequest::with_attribute_id()`].
    //
    // TODO: Add examples like forAsyncMonitoredItemBuilder.
    #[must_use]
    pub fn attribute_id(
        self,
        attribute_id: ua::AttributeId,
    ) -> MonitoredItemCreateRequestBuilder<Unknown> {
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

        MonitoredItemCreateRequestBuilder {
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
    #[expect(
        clippy::missing_const_for_fn,
        reason = "Might become non-const by internal changes in the future."
    )]
    pub fn sampling_interval(mut self, sampling_interval: Option<Duration>) -> Self {
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
    #[expect(
        clippy::missing_const_for_fn,
        reason = "Might become non-const by internal changes in the future."
    )]
    pub fn queue_size(mut self, queue_size: u32) -> Self {
        self.queue_size = Some(queue_size);
        self
    }

    /// Sets discard policy.
    ///
    /// Default value is `true`.
    ///
    /// See [`ua::MonitoringParameters::with_discard_oldest()`].
    #[must_use]
    #[expect(
        clippy::missing_const_for_fn,
        reason = "Might become non-const by internal changes in the future."
    )]
    pub fn discard_oldest(mut self, discard_oldest: bool) -> Self {
        self.discard_oldest = Some(discard_oldest);
        self
    }

    /// Gets all node ids.
    #[must_use]
    // TODO: Change to const fn after bumping MSRV from 1.85 to 1.87.
    pub fn node_ids(&self) -> &[ua::NodeId] {
        self.node_ids.as_slice()
    }

    #[must_use]
    pub fn build(self, subscription_id: ua::SubscriptionId) -> ua::CreateMonitoredItemsRequest {
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

// TODO: Remove with deprecated `MonitoredItemBuilder`.
#[cfg(feature = "tokio")]
impl<K: MonitoredItemKind> From<crate::async_monitored_item::MonitoredItemBuilder<K>>
    for MonitoredItemCreateRequestBuilder<K>
{
    fn from(deprecated: crate::async_monitored_item::MonitoredItemBuilder<K>) -> Self {
        let crate::async_monitored_item::MonitoredItemBuilder {
            node_ids,
            attribute_id,
            monitoring_mode,
            sampling_interval,
            filter,
            queue_size,
            discard_oldest,
            _kind: _,
        } = deprecated;
        Self {
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
