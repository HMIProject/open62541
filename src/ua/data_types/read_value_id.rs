use open62541_sys::UA_AttributeId;

use crate::{DataType as _, ua};

crate::data_type!(ReadValueId);
crate::member_accessors!(ReadValueId {
    nodeId: &ua::NodeId,
    #[skip(uses = Self::attribute_id)]
    attributeId: ua::AttributeId,
    dataEncoding: &ua::QualifiedName,
    indexRange: &ua::String
});

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

    #[must_use]
    pub const fn attribute_id(&self) -> ua::AttributeId {
        ua::AttributeId(UA_AttributeId(self.0.attributeId))
    }
}
