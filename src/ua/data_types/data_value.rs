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
    pub fn value(&self) -> Option<&ua::Variant> {
        self.0
            .hasValue()
            .then(|| ua::Variant::raw_ref(&self.0.value))
    }

    #[must_use]
    pub fn status_code(&self) -> Option<ua::StatusCode> {
        self.0
            .hasStatus()
            .then(|| ua::StatusCode::new(self.0.status))
    }
}
