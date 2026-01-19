use crate::data_types::String;

// [Part 3: 8.3 QualifiedName](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.3)
// [Part 6: 5.1.12 QualifiedName, NodeId and ExpandedNodeId String Encoding](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.12)
#[derive(Debug, Clone)]
pub struct QualifiedName(pub u16, pub String);
