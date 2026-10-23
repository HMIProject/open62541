use crate::{ServiceResponse, ua};

crate::data_type!(TranslateBrowsePathsToNodeIdsResponse);
crate::member_accessors!(TranslateBrowsePathsToNodeIdsResponse {
    responseHeader: &ua::ResponseHeader,
    results: [ua::BrowsePathResult],
    diagnosticInfos: [ua::DiagnosticInfo]
});

impl ServiceResponse for TranslateBrowsePathsToNodeIdsResponse {
    type Request = ua::TranslateBrowsePathsToNodeIdsRequest;

    fn response_header(&self) -> &ua::ResponseHeader {
        Self::response_header(self)
    }
}
