use open62541_sys::UA_BrowsePathTarget;

use crate::data_type::DataType;
use crate::ua;

crate::data_type!(BrowsePathTarget);

impl BrowsePathTarget {
    pub fn new(browse_path_target: UA_BrowsePathTarget) -> Self {
        Self(browse_path_target)
    }
    pub fn get_target_id(&self) -> ua::ExpandedNodeId {
        ua::ExpandedNodeId::clone_raw(&self.0.targetId)
    }
}
