use crate::{ua, DataType as _};

crate::data_type!(BrowsePathTarget);

impl BrowsePathTarget {
    #[must_use]
    pub fn target_id(&self) -> &ua::ExpandedNodeId {
        ua::ExpandedNodeId::raw_ref(&self.0.targetId)
    }
}
