use crate::{ua, DataType as _};

crate::data_type!(ReadValueId, UA_ReadValueId, UA_TYPES_READVALUEID);

impl ReadValueId {
    #[must_use]
    pub fn with_node_id(mut self, node_id: &ua::NodeId) -> Self {
        node_id.clone_into_raw(&mut self.0.nodeId);
        self
    }

    #[must_use]
    pub fn with_attribute_id(mut self, attribute_id: &ua::AttributeId) -> Self {
        self.0.attributeId = attribute_id.as_u32();
        self
    }
}
