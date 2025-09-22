use crate::{FilterOperand, ua};

crate::data_type!(ElementOperand);

impl FilterOperand for ElementOperand {
    fn to_extension_object(&self) -> ua::ExtensionObject {
        ua::ExtensionObject::new(self)
    }
}
