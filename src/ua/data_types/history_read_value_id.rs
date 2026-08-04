use crate::{DataType as _, ua};

crate::data_type!(HistoryReadValueId);

impl HistoryReadValueId {
    #[must_use]
    pub fn with_node_id(mut self, node_id: &ua::NodeId) -> Self {
        node_id.clone_into_raw(&mut self.0.nodeId);
        self
    }
}
