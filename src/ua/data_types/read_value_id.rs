use std::ptr;

use open62541_sys::{UA_NodeId_clear, UA_ReadValueId, UA_TYPES_READVALUEID};

use crate::ua;

ua::data_type!(ReadValueId, UA_ReadValueId, UA_TYPES_READVALUEID);

impl ua::DataType for ReadValueId {
    type Inner = UA_ReadValueId;

    const INNER: u32 = UA_TYPES_READVALUEID;

    fn as_ptr(&self) -> *const Self::Inner {
        ReadValueId::as_ptr(self)
    }
}

impl ReadValueId {
    #[must_use]
    pub fn node_id(mut self, node_id: &ua::NodeId) -> Self {
        let node_id = node_id.clone();

        unsafe { UA_NodeId_clear(ptr::addr_of_mut!(self.0.nodeId)) }
        self.0.nodeId = node_id.into_inner();

        self
    }

    #[must_use]
    pub fn attribute_id(mut self, attribute_id: u32) -> Self {
        self.0.attributeId = attribute_id;
        self
    }
}
