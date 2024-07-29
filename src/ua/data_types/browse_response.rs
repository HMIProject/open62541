use crate::{ua, ServiceResponse};

crate::data_type!(BrowseResponse);

impl BrowseResponse {
    #[must_use]
    pub fn results(&self) -> Option<ua::Array<ua::BrowseResult>> {
        // TODO: Adjust signature to return non-owned value instead.
        ua::Array::from_raw_parts(self.0.resultsSize, self.0.results)
    }
}

impl ServiceResponse for BrowseResponse {
    type Request = ua::BrowseRequest;

    fn service_result(&self) -> ua::StatusCode {
        ua::StatusCode::new(self.0.responseHeader.serviceResult)
    }
}
