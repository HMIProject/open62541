use crate::{ua, DataType as _};

crate::data_type!(ExpandedNodeId);

impl ExpandedNodeId {
    #[must_use]
    pub fn node_id(&self) -> &ua::NodeId {
        ua::NodeId::raw_ref(&self.0.nodeId)
    }

    #[must_use]
    pub fn namespace_uri(&self) -> &ua::String {
        ua::String::raw_ref(&self.0.namespaceUri)
    }

    #[must_use]
    pub fn server_index(&self) -> u32 {
        self.0.serverIndex
    }
}
