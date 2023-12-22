use crate::{ua, DataType as _};

crate::data_type!(ExpandedNodeId);

impl ExpandedNodeId {
    #[must_use]
    pub fn node_id(&self) -> &ua::NodeId {
        // SAFETY: There is no mutable reference to the inner value.
        unsafe { ua::NodeId::raw_ref(&self.0.nodeId) }
    }
}
