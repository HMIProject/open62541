use crate::{ua, DataType, Result};

crate::data_type!(DataValue);

impl DataValue {
    #[must_use]
    pub fn new(value: ua::Variant) -> Self {
        let mut inner = ua::DataValue::init();
        value.move_into_raw(&mut inner.0.value);
        inner.0.set_hasValue(true);
        debug_assert!(!inner.0.hasStatus());
        debug_assert!(!inner.0.hasSourceTimestamp());
        debug_assert!(!inner.0.hasServerTimestamp());
        debug_assert!(!inner.0.hasSourcePicoseconds());
        debug_assert!(!inner.0.hasServerPicoseconds());
        inner
    }

    #[must_use]
    pub fn with_value(mut self, value: &ua::Variant) -> Self {
        value.clone_into_raw(&mut self.0.value);
        self.0.set_hasValue(true);
        self
    }

    #[must_use]
    pub fn with_status_code(mut self, status_code: &ua::StatusCode) -> Self {
        status_code.clone_into_raw(&mut self.0.status);
        self.0.set_hasStatus(true);
        self
    }

    /// Gets value.
    ///
    /// This returns the value as [`ua::Variant`] if it is set. Returns `None` when the `DataValue`
    /// holds no value.
    #[must_use]
    pub fn value(&self) -> Option<&ua::Variant> {
        self.0
            .hasValue()
            .then(|| ua::Variant::raw_ref(&self.0.value))
    }

    #[must_use]
    pub fn source_timestamp(&self) -> Option<&ua::DateTime> {
        self.0
            .hasSourceTimestamp()
            .then(|| ua::DateTime::raw_ref(&self.0.sourceTimestamp))
    }

    #[must_use]
    pub fn server_timestamp(&self) -> Option<&ua::DateTime> {
        self.0
            .hasServerTimestamp()
            .then(|| ua::DateTime::raw_ref(&self.0.serverTimestamp))
    }

    #[must_use]
    pub fn source_picoseconds(&self) -> Option<u16> {
        self.0
            .hasSourcePicoseconds()
            .then_some(self.0.sourcePicoseconds)
    }

    #[must_use]
    pub fn server_picoseconds(&self) -> Option<u16> {
        self.0
            .hasServerPicoseconds()
            .then_some(self.0.serverPicoseconds)
    }

    #[must_use]
    pub fn status_code(&self) -> Option<ua::StatusCode> {
        self.0
            .hasStatus()
            .then(|| ua::StatusCode::new(self.0.status))
    }

    pub(crate) fn to_generic<T: DataType>(&self) -> Result<crate::DataValue<T>> {
        crate::DataValue::new(self)
    }
}
