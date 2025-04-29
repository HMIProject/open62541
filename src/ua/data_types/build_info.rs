use crate::{ua, DataType as _};

crate::data_type!(BuildInfo);

impl BuildInfo {
    #[must_use]
    pub const fn build_date(&self) -> i64 {
        self.0.buildDate
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
