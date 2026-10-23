use crate::{DataType as _, Error, ua};

crate::data_type!(LocalizedText);
crate::member_accessors!(LocalizedText {
    locale: &ua::String,
    text: &ua::String
});

impl LocalizedText {
    /// Creates localized text from locale and text.
    ///
    /// # Errors
    ///
    /// The strings must not contain any NUL bytes.
    pub fn new(locale: &str, text: &str) -> Result<Self, Error> {
        Self::init().with_locale(locale)?.with_text(text)
    }

    /// # Errors
    ///
    /// The string must not contain any NUL bytes.
    pub fn with_locale(mut self, locale: &str) -> Result<Self, Error> {
        ua::String::new(locale)?.move_into_raw(&mut self.0.locale);
        Ok(self)
    }

    /// # Errors
    ///
    /// The string must not contain any NUL bytes.
    pub fn with_text(mut self, text: &str) -> Result<Self, Error> {
        ua::String::new(text)?.move_into_raw(&mut self.0.text);
        Ok(self)
    }
}
