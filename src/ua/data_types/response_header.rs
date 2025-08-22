use crate::ua;

crate::data_type!(ResponseHeader);

impl ResponseHeader {
    /// Returns the resulting status.
    #[must_use]
    pub const fn service_result(&self) -> ua::StatusCode {
        ua::StatusCode::new(self.0.serviceResult)
    }
}
