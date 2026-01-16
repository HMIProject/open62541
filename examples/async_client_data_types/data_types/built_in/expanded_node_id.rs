use super::{NodeId, String};

// [Part 6: 5.2.2.10 ExpandedNodeId](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.10)
pub struct ExpandedNodeId(NodeId, String, u32);
