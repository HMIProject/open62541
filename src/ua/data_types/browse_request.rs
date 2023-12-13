use crate::{ua, ServiceRequest};

crate::data_type!(BrowseRequest, UA_BrowseRequest, UA_TYPES_BROWSEREQUEST);

impl BrowseRequest {
    #[must_use]
    pub fn with_nodes_to_browse(mut self, nodes_to_browse: &[ua::BrowseDescription]) -> Self {
        let array = ua::Array::from_slice(nodes_to_browse);

        // Make sure to clean up any previous value in target.
        let _unused = ua::Array::<ua::BrowseDescription>::from_raw_parts(
            self.0.nodesToBrowse,
            self.0.nodesToBrowseSize,
        );

        // Transfer ownership from `array` into `self`.
        let (size, ptr) = array.into_raw_parts();
        self.0.nodesToBrowseSize = size;
        self.0.nodesToBrowse = ptr;

        self
    }
}

impl ServiceRequest for BrowseRequest {
    type Response = ua::BrowseResponse;
}
