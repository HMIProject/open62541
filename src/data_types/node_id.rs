use std::{ffi::CString, fmt};

use log::debug;
use open62541_sys::{
    UA_NodeId, UA_NodeId_clear, UA_NodeId_print, UA_NODEID_NUMERIC, UA_NODEID_STRING_ALLOC,
    UA_STATUSCODE_GOOD,
};

use crate::ua;

pub struct NodeId(UA_NodeId);

impl NodeId {
    pub fn new_numeric(ns_index: u16, identifier: u32) -> Option<Self> {
        debug!("Creating numeric UI_NodeId");

        let node_id = unsafe { UA_NODEID_NUMERIC(ns_index, identifier) };

        Some(NodeId(node_id))
    }

    pub fn new_string(ns_index: u16, chars: &str) -> Option<Self> {
        debug!("Creating string UI_NodeId");

        let chars = CString::new(chars).ok()?;

        // Technically, string allocation may fail but `UA_NODEID_STRING_ALLOC` doesn't tell us that
        // when it happens. Instead, we end up with a well-defined node ID that has an empty string.
        let node_id = unsafe { UA_NODEID_STRING_ALLOC(ns_index, chars.as_ptr()) };

        Some(NodeId(node_id))
    }

    pub const fn as_ptr(&self) -> *const UA_NodeId {
        &self.0 as *const UA_NodeId
    }
}

impl Drop for NodeId {
    fn drop(&mut self) {
        debug!("Dropping UI_NodeId");

        let node_id = &mut self.0 as *mut UA_NodeId;

        unsafe { UA_NodeId_clear(node_id) }
    }
}

impl fmt::Debug for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = ua::String::new();

        let result = unsafe { UA_NodeId_print(self.as_ptr(), output.as_mut()) };

        if result != UA_STATUSCODE_GOOD {
            return f.write_str("NodeId");
        }

        match output.to_string() {
            Some(string) => f.write_str(&string),
            None => f.write_str("NodeId"),
        }
    }
}
