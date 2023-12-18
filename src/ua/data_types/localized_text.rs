use crate::ua;

crate::data_type!(LocalizedText, UA_LocalizedText, UA_TYPES_LOCALIZEDTEXT);

impl LocalizedText {
    #[must_use]
    pub fn locale(&self) -> ua::String {
        ua::String::from_ref(&self.0.locale)
    }

    #[must_use]
    pub fn text(&self) -> ua::String {
        ua::String::from_ref(&self.0.text)
    }
}
