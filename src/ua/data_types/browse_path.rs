use crate::{DataType as _, ua};

crate::data_type!(BrowsePath);

impl BrowsePath {
    #[must_use]
    pub fn with_starting_node(mut self, node_id: &ua::NodeId) -> Self {
        node_id.clone_into_raw(&mut self.0.startingNode);
        self
    }

    #[must_use]
    pub fn with_relative_path(mut self, relative_path: &ua::RelativePath) -> Self {
        relative_path.clone_into_raw(&mut self.0.relativePath);
        self
    }
}
