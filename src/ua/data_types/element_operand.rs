use crate::{FilterOperand, ua};

crate::data_type!(ElementOperand);
crate::member_accessors!(ElementOperand { index: u32 });

impl FilterOperand for ElementOperand {
    fn to_extension_object(&self) -> ua::ExtensionObject {
        ua::ExtensionObject::new(self)
    }
}
