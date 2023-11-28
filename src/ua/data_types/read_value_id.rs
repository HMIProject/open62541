use std::ptr;

use open62541_sys::UA_NodeId_clear;

use crate::ua;

crate::data_type!(ReadValueId, UA_ReadValueId, UA_TYPES_READVALUEID);

impl ReadValueId {
    #[must_use]
    pub fn with_node_id(mut self, node_id: &ua::NodeId) -> Self {
        let node_id = node_id.clone();

        // Make sure to properly clean up any previous node ID here.
        unsafe { UA_NodeId_clear(ptr::addr_of_mut!(self.0.nodeId)) }
        self.0.nodeId = node_id.into_inner();

        self
    }

    #[must_use]
    pub fn with_attribute_id(mut self, attribute_id: u32) -> Self {
        self.0.attributeId = attribute_id;
        self
    }
}
