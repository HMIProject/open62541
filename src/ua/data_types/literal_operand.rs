use crate::{DataType as _, FilterOperand, ua};

crate::data_type!(LiteralOperand);

impl LiteralOperand {
    #[must_use]
    pub fn new(value: ua::Variant) -> Self {
        let mut inner = Self::init();
        value.move_into_raw(&mut inner.0.value);
        inner
    }
}

impl FilterOperand for LiteralOperand {
    fn to_extension_object(&self) -> ua::ExtensionObject {
        ua::ExtensionObject::new(self)
    }
}
