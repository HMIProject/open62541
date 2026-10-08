use crate::{DataType, ServiceRequest, ua};

crate::data_type!(BrowseRequest);
crate::member_accessors!(BrowseRequest {
    nodesToBrowse: [ua::BrowseDescription],
    requestedMaxReferencesPerNode: u32,
    view: &ua::ViewDescription,
    requestHeader: &mut ua::RequestHeader
});

impl BrowseRequest {
    #[must_use]
    pub fn with_nodes_to_browse(mut self, nodes_to_browse: &[ua::BrowseDescription]) -> Self {
        let array = ua::Array::from_slice(nodes_to_browse);
        array.move_into_raw(&mut self.0.nodesToBrowseSize, &mut self.0.nodesToBrowse);
        self
    }

    #[must_use]
    pub const fn with_requested_max_references_per_node(
        mut self,
        requested_max_references_per_node: u32,
    ) -> Self {
        self.0.requestedMaxReferencesPerNode = requested_max_references_per_node;
        self
    }

    #[must_use]
    pub fn with_view(mut self, view: &ua::ViewDescription) -> Self {
        view.clone_into_raw(&mut self.0.view);
        self
    }
}

impl ServiceRequest for BrowseRequest {
    type Response = ua::BrowseResponse;

    fn request_header(&self) -> &ua::RequestHeader {
        ua::RequestHeader::raw_ref(&self.0.requestHeader)
    }

    fn request_header_mut(&mut self) -> &mut ua::RequestHeader {
        ua::RequestHeader::raw_mut(&mut self.0.requestHeader)
    }
}
