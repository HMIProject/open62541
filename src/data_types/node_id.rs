use std::{ffi::CString, fmt, ptr};

use open62541_sys::{
    UA_NodeId, UA_NodeIdType_UA_NODEIDTYPE_NUMERIC, UA_NodeIdType_UA_NODEIDTYPE_STRING,
    UA_NodeId_clear, UA_NodeId_print, UA_NODEID_NULL, UA_NODEID_NUMERIC, UA_NODEID_STRING_ALLOC,
    UA_STATUSCODE_GOOD,
};

use crate::ua;

pub struct NodeId(UA_NodeId);

impl NodeId {
    #[must_use]
    pub fn new() -> Self {
        Self(unsafe { UA_NODEID_NULL })
    }

    #[must_use]
    pub fn new_numeric(ns_index: u16, identifier: u32) -> Option<Self> {
        let node_id = unsafe { UA_NODEID_NUMERIC(ns_index, identifier) };

        debug_assert_eq!(node_id.identifierType, UA_NodeIdType_UA_NODEIDTYPE_NUMERIC);

        if node_id.identifierType != UA_NodeIdType_UA_NODEIDTYPE_NUMERIC {
            return None;
        }

        Some(Self(node_id))
    }

    #[must_use]
    pub fn new_string(ns_index: u16, chars: &str) -> Option<Self> {
        let chars = CString::new(chars).ok()?;

        // Technically, string allocation may fail but `UA_NODEID_STRING_ALLOC` doesn't tell us that
        // when it happens. Instead, we end up with a well-defined node ID that has an empty string.
        let node_id = unsafe { UA_NODEID_STRING_ALLOC(ns_index, chars.as_ptr()) };

        debug_assert_eq!(node_id.identifierType, UA_NodeIdType_UA_NODEIDTYPE_STRING);

        if node_id.identifierType != UA_NodeIdType_UA_NODEIDTYPE_STRING
            || unsafe { node_id.identifier.string.length } == 0
        {
            return None;
        }

        Some(Self(node_id))
    }

    #[must_use]
    pub const fn as_ptr(&self) -> *const UA_NodeId {
        ptr::addr_of!(self.0)
    }
}

impl Drop for NodeId {
    fn drop(&mut self) {
        let node_id = ptr::addr_of_mut!(self.0);

        unsafe { UA_NodeId_clear(node_id) }
    }
}

impl Default for NodeId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = ua::String::new();

        let result = unsafe { UA_NodeId_print(self.as_ptr(), output.as_mut()) };

        if result != UA_STATUSCODE_GOOD {
            return f.write_str("NodeId");
        }

        match output.as_str() {
            Some(str) => f.write_str(str),
            None => f.write_str("NodeId"),
        }
    }
}
