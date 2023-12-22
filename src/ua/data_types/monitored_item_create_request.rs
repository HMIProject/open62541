use open62541_sys::UA_MonitoredItemCreateRequest_default;

use crate::{ua, DataType as _};

crate::data_type!(MonitoredItemCreateRequest);

impl MonitoredItemCreateRequest {
    #[must_use]
    // TODO: Find better name for this method.
    pub fn init_node_id(node_id: &ua::NodeId) -> Self {
        let inner = unsafe { UA_MonitoredItemCreateRequest_default(node_id.clone().into_raw()) };
        Self(inner)
    }
}
