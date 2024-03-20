use std::str::FromStr;

use crate::{ua, DataType as _, Error};

crate::data_type!(LocalizedText);

impl LocalizedText {
    /// # Errors
    ///
    /// The string must not contain any NUL bytes.
    pub fn with_locale(mut self, locale: &str) -> Result<Self, Error> {
        ua::String::from_str(locale)?.move_into_raw(&mut self.0.locale);
        Ok(self)
    }

    /// # Errors
    ///
    /// The string must not contain any NUL bytes.
    pub fn with_text(mut self, text: &str) -> Result<Self, Error> {
        ua::String::from_str(text)?.move_into_raw(&mut self.0.text);
        Ok(self)
    }

    #[must_use]
    pub fn locale(&self) -> &ua::String {
        ua::String::raw_ref(&self.0.locale)
    }

    #[must_use]
    pub fn text(&self) -> &ua::String {
        ua::String::raw_ref(&self.0.text)
    }
}

impl TryFrom<(&str, &str)> for LocalizedText {
    type Error = Error;

    fn try_from((locale, text): (&str, &str)) -> Result<Self, Self::Error> {
        Self::init().with_locale(locale)?.with_text(text)
    }
}
