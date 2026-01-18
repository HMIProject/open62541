use crate::{
    binary::BinaryReader,
    data_types::{ExpandedNodeId, Index, String},
};

use super::node_id::{NodeIdWithDataEncodingFlags, read_node_id_with_encoding_flags};

// [Part 6: 5.2.2.10 ExpandedNodeId](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.10)
impl BinaryReader for ExpandedNodeId {
    fn read(data: &mut bytes::Bytes) -> Self {
        let NodeIdWithDataEncodingFlags {
            node_id,
            namespace_uri,
            server_index,
        } = read_node_id_with_encoding_flags(data);
        let namespace_uri = namespace_uri
            .then(|| String::read(data))
            .unwrap_or_else(String::null);
        let server_index = server_index
            .then(|| Index::read(data))
            .unwrap_or_else(Index::zero);
        Self(server_index, namespace_uri, node_id)
    }
}
