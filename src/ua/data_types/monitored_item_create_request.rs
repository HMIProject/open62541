use open62541_sys::{UA_MonitoredItemCreateRequest_default, UA_NODEID_NUMERIC};

use crate::{ua, DataType as _};

crate::data_type!(MonitoredItemCreateRequest);

impl MonitoredItemCreateRequest {
    #[must_use]
    pub fn with_node_id(mut self, node_id: &ua::NodeId) -> Self {
        node_id.clone_into_raw(&mut self.0.itemToMonitor.nodeId);
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
