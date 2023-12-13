use crate::ua;

crate::data_type!(
    BrowseDescription,
    UA_BrowseDescription,
    UA_TYPES_BROWSEDESCRIPTION
);

impl BrowseDescription {
    #[must_use]
    pub fn with_node_id(mut self, node_id: &ua::NodeId) -> Self {
        node_id.clone_into(&mut self.0.nodeId);
        self
    }

    #[must_use]
    pub fn with_result_mask(mut self, result_mask: ua::ResultMask) -> Self {
        self.0.resultMask = result_mask.into_inner();
        self
    }
}
