use crate::data_types::{ByteString, NodeId, Structure, XmlElement};

// [Part 6: 5.1.8 ExtensionObject](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.8)
#[derive(Debug, Clone)]
pub enum ExtensionObject {
    Null(NodeId),
    ByteString(NodeId, ByteString),
    XmlElement(NodeId, XmlElement),
    Structure(NodeId, Structure),
}

impl ExtensionObject {
    pub fn type_id(&self) -> &NodeId {
        match self {
            Self::Null(type_id) => type_id,
            Self::ByteString(type_id, _) => type_id,
            Self::XmlElement(type_id, _) => type_id,
            Self::Structure(type_id, _) => type_id,
        }
    }
}
