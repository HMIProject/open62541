use bytes::Bytes;

use crate::{
    binary::BinaryReader,
    data_types::{Byte, ByteString, Guid, NodeId, String, UInt16, UInt32},
};

// [Part 6: 5.2.2.9 NodeId](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.9)
impl BinaryReader for NodeId {
    fn read(data: &mut Bytes) -> Self {
        let NodeIdWithDataEncodingFlags {
            has_namespace_uri,
            has_server_index,
            node_id,
        } = read_node_id_with_data_encoding_flags(data);

        assert!(!has_namespace_uri);
        assert!(!has_server_index);

        node_id
    }
}

pub(super) struct NodeIdWithDataEncodingFlags {
    pub(super) has_namespace_uri: bool,
    pub(super) has_server_index: bool,
    pub(super) node_id: NodeId,
}

// [Part 6: 5.2.2.9 NodeId](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.9)
pub(super) fn read_node_id_with_data_encoding_flags(
    data: &mut Bytes,
) -> NodeIdWithDataEncodingFlags {
    let data_encoding = Byte::read(data);

    let has_namespace_uri = (data_encoding.0 & 0x40) != 0x00;
    let has_server_index = (data_encoding.0 & 0x80) != 0x00;

    let node_id = match data_encoding.0 {
        0x00 => {
            let identifier = Byte::read(data);
            NodeId::Numeric(0, u32::from(identifier.0))
        }
        0x01 => {
            let namespace = Byte::read(data);
            let identifier = UInt16::read(data);
            NodeId::Numeric(u16::from(namespace.0), u32::from(identifier.0))
        }
        0x02 => {
            let namespace = UInt16::read(data);
            let identifier = UInt32::read(data);
            NodeId::Numeric(namespace.0, identifier.0)
        }
        0x03 => {
            let namespace = UInt16::read(data);
            let identifier = String::read(data);
            NodeId::String(namespace.0, identifier)
        }
        0x04 => {
            let namespace = UInt16::read(data);
            let identifier = Guid::read(data);
            NodeId::Guid(namespace.0, identifier)
        }
        0x05 => {
            let namespace = UInt16::read(data);
            let identifier = ByteString::read(data);
            NodeId::Opaque(namespace.0, identifier)
        }
        _ => {
            panic!();
        }
    };

    NodeIdWithDataEncodingFlags {
        has_namespace_uri,
        has_server_index,
        node_id,
    }
}
