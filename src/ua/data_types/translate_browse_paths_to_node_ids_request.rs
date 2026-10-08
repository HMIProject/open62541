use crate::{ServiceRequest, ua};

crate::data_type!(TranslateBrowsePathsToNodeIdsRequest);
crate::member_accessors!(TranslateBrowsePathsToNodeIdsRequest {
    browsePaths: [ua::BrowsePath],
    requestHeader: &mut ua::RequestHeader
});

impl TranslateBrowsePathsToNodeIdsRequest {
    #[must_use]
    pub fn with_browse_paths(mut self, paths: &[ua::BrowsePath]) -> Self {
        ua::Array::from_slice(paths)
            .move_into_raw(&mut self.0.browsePathsSize, &mut self.0.browsePaths);

        self
    }
}

impl ServiceRequest for TranslateBrowsePathsToNodeIdsRequest {
    type Response = ua::TranslateBrowsePathsToNodeIdsResponse;

    fn request_header(&self) -> &ua::RequestHeader {
        Self::request_header(self)
    }

    fn request_header_mut(&mut self) -> &mut ua::RequestHeader {
        Self::request_header_mut(self)
    }
}
