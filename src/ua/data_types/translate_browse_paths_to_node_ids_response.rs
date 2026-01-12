use crate::{DataType as _, ServiceResponse, ua};

crate::data_type!(TranslateBrowsePathsToNodeIdsResponse);

impl TranslateBrowsePathsToNodeIdsResponse {
    #[must_use]
    pub fn results(&self) -> Option<ua::Array<ua::BrowsePathResult>> {
        ua::Array::from_raw_parts(self.0.resultsSize, self.0.results)
    }
}

impl ServiceResponse for TranslateBrowsePathsToNodeIdsResponse {
    type Request = ua::TranslateBrowsePathsToNodeIdsRequest;

    fn response_header(&self) -> &ua::ResponseHeader {
        ua::ResponseHeader::raw_ref(&self.0.responseHeader)
    }
}
