use crate::{ua, FilterOperand};

crate::data_type!(AttributeOperand);

impl FilterOperand for AttributeOperand {
    fn to_extension_object(&self) -> ua::ExtensionObject {
        ua::ExtensionObject::new(self)
    }
}
