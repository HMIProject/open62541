use crate::{ua, DataType as _};

crate::data_type!(DataValue);

impl DataValue {
    #[must_use]
    pub fn with_value(mut self, value: &ua::Variant) -> Self {
        value.clone_into_raw(&mut self.0.value);
        self.0.set_hasValue(true);
        self
    }

    #[must_use]
    pub fn value(&self) -> Option<ua::Variant> {
        // TODO: Adjust signature to return non-owned value instead.
        if self.0.hasValue() {
            Some(ua::Variant::clone_raw(&self.0.value))
        } else {
            None
        }
    }
}
