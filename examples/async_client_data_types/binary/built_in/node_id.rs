use bytes::Bytes;

use crate::{
    binary::BinaryReader,
    data_types::{Byte, ByteString, Guid, NodeId, String, UInt16, UInt32},
};

// [Part 6: 5.2.2.9 NodeId](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.9)
impl BinaryReader for NodeId {
    fn read(data: &mut Bytes) -> Self {
        let data_encoding = Byte::read(data);
        assert!(data_encoding.0 & 0x80 == 0x00);
        assert!(data_encoding.0 & 0x40 == 0x00);
        match data_encoding.0 {
            0x00 => {
                let identifier = Byte::read(data);
                Self::Numeric(0, u32::from(identifier.0))
            }
            0x01 => {
                let namespace = Byte::read(data);
                let identifier = UInt16::read(data);
                Self::Numeric(u16::from(namespace.0), u32::from(identifier.0))
            }
            0x02 => {
                let namespace = UInt16::read(data);
                let identifier = UInt32::read(data);
                Self::Numeric(namespace.0, identifier.0)
            }
            0x03 => {
                let namespace = UInt16::read(data);
                let identifier = String::read(data);
                Self::String(namespace.0, identifier)
            }
            0x04 => {
                let namespace = UInt16::read(data);
                let identifier = Guid::read(data);
                Self::Guid(namespace.0, identifier)
            }
            0x05 => {
                let namespace = UInt16::read(data);
                let identifier = ByteString::read(data);
                Self::Opaque(namespace.0, identifier)
            }
            _ => {
                panic!();
            }
        }
    }
}
