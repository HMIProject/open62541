use crate::ua;

crate::data_type!(
    BrowseDescription,
    UA_BrowseDescription,
    UA_TYPES_BROWSEDESCRIPTION
);

impl BrowseDescription {
    #[must_use]
    pub fn with_node_id(mut self, node_id: &ua::NodeId) -> Self {
        let node_id = node_id.clone();

        // Make sure to clean up any previous value in target.
        let _unused = ua::NodeId::new(self.0.nodeId);

        // Transfer ownership from `node_id` into `self`.
        self.0.nodeId = node_id.into_inner();

        self
    }

    #[must_use]
    pub fn with_result_mask(mut self, result_mask: ua::ResultMask) -> Self {
        self.0.resultMask = result_mask.into_inner();
        self
    }
}
