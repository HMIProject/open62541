use crate::data_types::{ByteString, Guid, String};

// [Part 3: 8.2 NodeId](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.2)
// [Part 5: 12.2.8 NodeId](https://reference.opcfoundation.org/Core/Part5/v105/docs/12.2.8)
// [Part 6: 5.1.12 QualifiedName, NodeId and ExpandedNodeId String Encoding](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.12)
// [Part 6: 5.2.2.9 NodeId](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.9)
#[derive(Debug, Clone)]
pub enum NodeId {
    Numeric(u16, u32),
    String(u16, String),
    Guid(u16, Guid),
    Opaque(u16, ByteString),
}

impl NodeId {
    // [Part 3: 8.2.4 Identifier value](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.2.4)
    #[must_use]
    pub fn is_null(&self) -> bool {
        match self {
            Self::Numeric(namespace_index, identifier) => *namespace_index == 0 && *identifier == 0,
            Self::String(namespace_index, identifier) => {
                *namespace_index == 0 && (identifier.is_null() || identifier.is_empty())
            }
            Self::Guid(namespace_index, identifier) => {
                *namespace_index == 0 && identifier.is_zero()
            }
            Self::Opaque(namespace_index, identifier) => {
                *namespace_index == 0 && (identifier.is_null() || identifier.is_empty())
            }
        }
    }

    #[must_use]
    pub fn as_ns0(&self) -> Option<u32> {
        if let NodeId::Numeric(namespace_index, identifier) = self
            && *namespace_index == 0
        {
            Some(*identifier)
        } else {
            None
        }
    }
}
