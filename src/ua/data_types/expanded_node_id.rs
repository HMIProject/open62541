use crate::{ua, DataType as _};

crate::data_type!(ExpandedNodeId);

impl ExpandedNodeId {
    #[must_use]
    pub fn node_id(&self) -> ua::NodeId {
        ua::NodeId::clone_raw(&self.0.nodeId)
    }
}
