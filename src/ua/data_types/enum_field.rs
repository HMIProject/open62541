use crate::{ua, DataType as _};

crate::data_type!(EnumField);

impl EnumField {
    #[must_use]
    pub fn description(&self) -> &ua::LocalizedText {
        ua::LocalizedText::raw_ref(&self.0.description)
    }

    #[must_use]
    pub fn display_name(&self) -> &ua::LocalizedText {
        ua::LocalizedText::raw_ref(&self.0.displayName)
    }

    #[must_use]
    pub fn name(&self) -> &ua::String {
        ua::String::raw_ref(&self.0.name)
    }

    #[must_use]
    pub const fn value(&self) -> i64 {
        self.0.value
    }
}
