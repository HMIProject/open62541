use crate::{DataType as _, ServiceRequest, ua};

crate::data_type!(TranslateBrowsePathsToNodeIdsRequest);

impl TranslateBrowsePathsToNodeIdsRequest {
    #[must_use]
    pub fn with_browse_paths(
        mut self,
        browse_paths: &[ua::BrowsePath]
    ) -> Self {
        let array = ua::Array::from_slice(browse_paths);
        array.move_into_raw(&mut self.0.browsePathsSize, &mut self.0.browsePaths);
        self
    }
}

impl ServiceRequest for TranslateBrowsePathsToNodeIdsRequest {
    type Response = ua::TranslateBrowsePathsToNodeIdsResponse;

    fn request_header(&self) -> &ua::RequestHeader {
        ua::RequestHeader::raw_ref(&self.0.requestHeader)
    }

    fn request_header_mut(&mut self) -> &mut ua::RequestHeader {
        ua::RequestHeader::raw_mut(&mut self.0.requestHeader)
    }
}
