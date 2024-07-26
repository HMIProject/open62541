use open62541_sys::{UA_NodeIdType, UA_EXPANDEDNODEID_NODEID, UA_EXPANDEDNODEID_NUMERIC};

use crate::{ua, DataType as _};

crate::data_type!(ExpandedNodeId);

impl ExpandedNodeId {
    /// Creates numeric expanded node ID.
    #[must_use]
    pub fn numeric(ns_index: u16, numeric: u32) -> Self {
        let inner = unsafe { UA_EXPANDEDNODEID_NUMERIC(ns_index, numeric) };
        debug_assert_eq!(
            inner.nodeId.identifierType,
            UA_NodeIdType::UA_NODEIDTYPE_NUMERIC,
            "new node ID should have numeric type"
        );

        Self(inner)
    }

    /// Creates expanded node ID from node ID.
    #[must_use]
    pub(crate) fn from_node_id(node_id: ua::NodeId) -> Self {
        // This passes ownership into the created expanded node ID.
        Self(unsafe { UA_EXPANDEDNODEID_NODEID(node_id.into_raw()) })
    }

    #[must_use]
    pub fn node_id(&self) -> &ua::NodeId {
        ua::NodeId::raw_ref(&self.0.nodeId)
    }

    #[must_use]
    pub fn namespace_uri(&self) -> &ua::String {
        ua::String::raw_ref(&self.0.namespaceUri)
    }

    #[must_use]
    pub const fn server_index(&self) -> u32 {
        self.0.serverIndex
    }
}
