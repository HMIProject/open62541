use crate::{FilterOperand, ua};

crate::data_type!(AttributeOperand);
crate::member_accessors!(AttributeOperand {
    indexRange: &ua::String,
    nodeId: &ua::NodeId,
    browsePath: &ua::RelativePath,
    alias: &ua::String,
    #[skip(uses = AttributeOperand::attribute_id)]
    attributeId: ua::AttributeId
});

impl AttributeOperand {
    #[must_use]
    pub const fn attribute_id(&self) -> ua::AttributeId {
        ua::AttributeId::from_u32(self.0.attributeId)
    }
}

impl FilterOperand for AttributeOperand {
    fn to_extension_object(&self) -> ua::ExtensionObject {
        ua::ExtensionObject::new(self)
    }
}
