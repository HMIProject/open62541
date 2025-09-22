use crate::{DataType as _, ua};

crate::data_type!(BuildInfo);

impl BuildInfo {
    #[must_use]
    pub fn build_date(&self) -> ua::DateTime {
        // SAFETY: The i64 value represents a valid UtcTime.
        unsafe { ua::DateTime::from_raw(self.0.buildDate) }
    }

    #[must_use]
    pub fn build_number(&self) -> &ua::String {
        ua::String::raw_ref(&self.0.buildNumber)
    }

    #[must_use]
    pub fn manufacturer_name(&self) -> &ua::String {
        ua::String::raw_ref(&self.0.manufacturerName)
    }

    #[must_use]
    pub fn product_name(&self) -> &ua::String {
        ua::String::raw_ref(&self.0.productName)
    }

    #[must_use]
    pub fn product_uri(&self) -> &ua::String {
        ua::String::raw_ref(&self.0.productUri)
    }

    #[must_use]
    pub fn software_version(&self) -> &ua::String {
        ua::String::raw_ref(&self.0.softwareVersion)
    }
}
