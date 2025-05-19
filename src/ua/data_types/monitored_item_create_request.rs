use std::time::Duration;

use open62541_sys::{UA_MonitoredItemCreateRequest_default, UA_NODEID_NUMERIC};

use crate::{ua, DataType as _, MonitoringFilter};

crate::data_type!(MonitoredItemCreateRequest);

impl MonitoredItemCreateRequest {
    /// Sets item to monitor.
    #[must_use]
    pub fn with_item_to_monitor(mut self, item_to_monitor: &ua::ReadValueId) -> Self {
        item_to_monitor.clone_into_raw(&mut self.0.itemToMonitor);
        self
    }

    /// Shortcut for setting node ID.
    ///
    /// See [`ua::ReadValueId::with_node_id()`].
    #[must_use]
    pub fn with_node_id(mut self, node_id: &ua::NodeId) -> Self {
        node_id.clone_into_raw(&mut self.0.itemToMonitor.nodeId);
        self
    }

    /// Shortcut for setting attribute ID.
    ///
    /// See [`ua::ReadValueId::with_attribute_id()`].
    #[must_use]
    pub fn with_attribute_id(mut self, attribute_id: &ua::AttributeId) -> Self {
        self.0.itemToMonitor.attributeId = attribute_id.as_u32();
        self
    }

    /// Sets monitoring mode.
    #[must_use]
    pub fn with_monitoring_mode(mut self, monitoring_mode: &ua::MonitoringMode) -> Self {
        monitoring_mode.clone_into_raw(&mut self.0.monitoringMode);
        self
    }

    /// Sets requested parameters.
    #[must_use]
    pub fn with_requested_parameters(
        mut self,
        requested_parameters: &ua::MonitoringParameters,
    ) -> Self {
        requested_parameters.clone_into_raw(&mut self.0.requestedParameters);
        self
    }

    /// Shortcut for setting sampling interval.
    ///
    /// See [`ua::MonitoringParameters::with_sampling_interval()`].
    #[must_use]
    pub fn with_sampling_interval(mut self, sampling_interval: Option<Duration>) -> Self {
        self.0.requestedParameters.samplingInterval =
            if let Some(sampling_interval) = sampling_interval {
                sampling_interval.as_secs_f64() * 1e3
            } else {
                -1.0
            };
        self
    }

    /// Shortcut for setting filter.
    #[must_use]
    pub fn with_filter(mut self, filter: &impl MonitoringFilter) -> Self {
        filter
            .to_extension_object()
            .move_into_raw(&mut self.0.requestedParameters.filter);
        self
    }

    /// Shortcut for setting requested size of the monitored item queue.
    ///
    /// See [`ua::MonitoringParameters::with_queue_size()`].
    #[must_use]
    pub const fn with_queue_size(mut self, queue_size: u32) -> Self {
        self.0.requestedParameters.queueSize = queue_size;
        self
    }

    /// Shortcut for setting discard policy.
    ///
    /// See [`ua::MonitoringParameters::with_discard_oldest()`].
    #[must_use]
    pub const fn with_discard_oldest(mut self, discard_oldest: bool) -> Self {
        self.0.requestedParameters.discardOldest = discard_oldest;
        self
    }

    #[cfg_attr(not(feature = "tokio"), expect(dead_code, reason = "unused"))]
    #[must_use]
    pub(crate) fn attribute_id(&self) -> ua::AttributeId {
        ua::AttributeId::from_u32(self.0.itemToMonitor.attributeId)
    }
}

impl Default for MonitoredItemCreateRequest {
    fn default() -> Self {
        // Use default node ID that does not own any additional data (such as dynamically allocated
        // string identifiers).
        let inner = unsafe { UA_MonitoredItemCreateRequest_default(UA_NODEID_NUMERIC(0, 0)) };
        Self(inner)
    }
}
