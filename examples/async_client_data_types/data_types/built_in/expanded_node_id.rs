use crate::data_types::{Index, NodeId, String};

// [Part 4: 7.16 ExpandedNodeId](https://reference.opcfoundation.org/Core/Part4/v105/docs/7.16)
// [Part 5: 12.3.9 ExpandedNodeId](https://reference.opcfoundation.org/Core/Part5/v105/docs/12.3.9)
// [Part 6: 5.1.12 QualifiedName, NodeId and ExpandedNodeId String Encoding](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.12)
// [Part 6: 5.2.2.10 ExpandedNodeId](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.10)
#[derive(Debug, Clone)]
pub struct ExpandedNodeId(pub Index, pub String, pub NodeId);
