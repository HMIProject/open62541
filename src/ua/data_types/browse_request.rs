use crate::{ua, ServiceRequest};

crate::data_type!(BrowseRequest);

impl BrowseRequest {
    #[must_use]
    pub fn with_nodes_to_browse(mut self, nodes_to_browse: &[ua::BrowseDescription]) -> Self {
        let array = ua::Array::from_slice(nodes_to_browse);
        array.move_into_raw(&mut self.0.nodesToBrowseSize, &mut self.0.nodesToBrowse);
        self
    }
}

impl ServiceRequest for BrowseRequest {
    type Response = ua::BrowseResponse;
}
