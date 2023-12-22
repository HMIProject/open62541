use crate::{ua, DataType as _};

crate::data_type!(LocalizedText);

impl LocalizedText {
    #[must_use]
    pub fn locale(&self) -> &ua::String {
        // SAFETY: There is no mutable reference to the inner value.
        unsafe { ua::String::raw_ref(&self.0.locale) }
    }

    #[must_use]
    pub fn text(&self) -> &ua::String {
        // SAFETY: There is no mutable reference to the inner value.
        unsafe { ua::String::raw_ref(&self.0.text) }
    }
}
