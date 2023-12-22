use crate::{ua, DataType as _};

crate::data_type!(LocalizedText);

impl LocalizedText {
    #[must_use]
    pub fn locale(&self) -> &ua::String {
        ua::String::raw_ref(&self.0.locale)
    }

    #[must_use]
    pub fn text(&self) -> &ua::String {
        ua::String::raw_ref(&self.0.text)
    }
}
