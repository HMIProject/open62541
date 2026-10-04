use crate::{DataType as _, ServiceRequest, ua};

crate::data_type!(TranslateBrowsePathsToNodeIdsRequest);

impl TranslateBrowsePathsToNodeIdsRequest {
    #[must_use]
    pub fn with_browse_paths(mut self, paths: &[ua::BrowsePath]) -> Self {
        ua::Array::from_slice(paths)
            .move_into_raw(&mut self.0.browsePathsSize, &mut self.0.browsePaths);

        self
    }

    #[must_use]
    pub fn browse_paths(&self) -> Option<&[ua::BrowsePath]> {
        unsafe { ua::Array::slice_from_raw_parts(self.0.browsePathsSize, self.0.browsePaths) }
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
