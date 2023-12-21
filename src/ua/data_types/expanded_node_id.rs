use crate::{ua, DataType as _};

crate::data_type!(ExpandedNodeId, UA_ExpandedNodeId, UA_TYPES_EXPANDEDNODEID);

impl ExpandedNodeId {
    #[must_use]
    pub fn node_id(&self) -> ua::NodeId {
        ua::NodeId::from_ref(&self.0.nodeId)
    }
}
