use open62541_sys::{UA_MonitoredItemCreateRequest_default, UA_NODEID_NUMERIC};

use crate::{ua, DataType as _};

crate::data_type!(MonitoredItemCreateRequest);

impl MonitoredItemCreateRequest {
    #[must_use]
    pub fn with_node_id(mut self, node_id: &ua::NodeId) -> Self {
        node_id.clone_into_raw(&mut self.0.itemToMonitor.nodeId);
        self
    }

    /// Sets item to monitor.
    #[must_use]
    pub fn with_item_to_monitor(mut self, item_to_monitor: &ua::ReadValueId) -> Self {
        item_to_monitor.clone_into_raw(&mut self.0.itemToMonitor);
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
}

impl Default for MonitoredItemCreateRequest {
    fn default() -> Self {
        // Use default node ID that does not own any additional data (such as dynamically allocated
        // string identifiers).
        let inner = unsafe { UA_MonitoredItemCreateRequest_default(UA_NODEID_NUMERIC(0, 0)) };
        Self(inner)
    }
}
