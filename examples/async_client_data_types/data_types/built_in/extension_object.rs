use super::{ByteString, NodeId, XmlElement};

// [Part 6: 5.1.8 ExtensionObject](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.8)
pub enum ExtensionObject {
    Null(NodeId),
    ByteString(NodeId, ByteString),
    XmlElement(NodeId, XmlElement),
}
