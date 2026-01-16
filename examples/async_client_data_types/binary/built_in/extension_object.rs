use bytes::Bytes;

use crate::{
    binary::BinaryReader,
    data_types::{Byte, ByteString, ExtensionObject, NodeId, XmlElement},
};

// [Part 6: 5.2.2.15 ExtensionObject](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.15)
impl BinaryReader for ExtensionObject {
    fn read(data: &mut Bytes) -> Self {
        let type_id = NodeId::read(data);
        let encoding = Byte::read(data);
        match encoding.0 {
            0x00 => Self::Null(type_id),
            0x01 => {
                let body = ByteString::read(data);
                Self::ByteString(type_id, body)
            }
            0x02 => {
                let body = XmlElement::read(data);
                Self::XmlElement(type_id, body)
            }
            _ => panic!(),
        }
    }
}
