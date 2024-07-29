use crate::ua;

crate::data_type!(BrowsePathResult);

impl BrowsePathResult {
    #[must_use]
    pub const fn status_code(&self) -> ua::StatusCode {
        ua::StatusCode::new(self.0.statusCode)
    }

    #[must_use]
    pub fn targets(&self) -> Option<ua::Array<ua::BrowsePathTarget>> {
        // TODO: Adjust signature to return non-owned value instead.
        ua::Array::from_raw_parts(self.0.targetsSize, self.0.targets)
    }
}
