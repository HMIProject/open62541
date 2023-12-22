use crate::{ua, DataType as _};

crate::data_type!(LocalizedText, UA_LocalizedText, UA_TYPES_LOCALIZEDTEXT);

impl LocalizedText {
    #[must_use]
    pub fn locale(&self) -> ua::String {
        ua::String::clone_raw(&self.0.locale)
    }

    #[must_use]
    pub fn text(&self) -> ua::String {
        ua::String::clone_raw(&self.0.text)
    }
}
