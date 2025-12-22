use crate::{DataType, FilterOperand, ua};

crate::data_type!(SimpleAttributeOperand);

impl SimpleAttributeOperand {
    #[must_use]
    pub fn with_type_definition_id(mut self, type_definition_id: ua::NodeId) -> Self {
        type_definition_id.move_into_raw(&mut self.0.typeDefinitionId);
        self
    }

    #[must_use]
    pub fn with_browse_path(mut self, browse_path: &[ua::QualifiedName]) -> Self {
        let array = ua::Array::from_slice(browse_path);
        array.move_into_raw(&mut self.0.browsePathSize, &mut self.0.browsePath);
        self
    }

    #[must_use]
    pub fn with_attribute_id(mut self, attribute_id: &ua::AttributeId) -> Self {
        self.0.attributeId = attribute_id.as_u32();
        self
    }

    #[must_use]
    pub fn with_index_range(mut self, index_range: ua::String) -> Self {
        index_range.move_into_raw(&mut self.0.indexRange);
        self
    }
}

impl FilterOperand for SimpleAttributeOperand {
    fn to_extension_object(&self) -> ua::ExtensionObject {
        ua::ExtensionObject::new(self)
    }
}
