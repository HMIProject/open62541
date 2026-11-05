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

    #[must_use]
    pub fn starting_node(&self) -> &ua::NodeId {
        ua::NodeId::raw_ref(&self.0.startingNode)
    }

    #[must_use]
    pub fn relative_path(&self) -> &ua::RelativePath {
        ua::RelativePath::raw_ref(&self.0.relativePath)
    }
}
