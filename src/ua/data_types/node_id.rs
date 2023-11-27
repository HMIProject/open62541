use std::{ffi::CString, fmt};

use open62541_sys::{
    UA_NodeIdType_UA_NODEIDTYPE_NUMERIC, UA_NodeIdType_UA_NODEIDTYPE_STRING, UA_NodeId_print,
    UA_NODEID_NUMERIC, UA_NODEID_STRING_ALLOC, UA_STATUSCODE_GOOD,
};

use crate::{ua, DataType};

ua::data_type!(NodeId, UA_NodeId, UA_TYPES_NODEID);

impl NodeId {
    /// Creates node ID for numeric identifier.
    #[must_use]
    pub fn new_numeric(ns_index: u16, numeric: u32) -> Self {
        let inner = unsafe { UA_NODEID_NUMERIC(ns_index, numeric) };
        debug_assert_eq!(inner.identifierType, UA_NodeIdType_UA_NODEIDTYPE_NUMERIC);
        Self(inner)
    }

    /// Creates node ID for string identifier.
    ///
    /// # Panics
    ///
    /// The string identifier must be a valid C string, i.e. it must not contain any NUL bytes.
    #[must_use]
    pub fn new_string(ns_index: u16, string: &str) -> Self {
        let string = CString::new(string).expect("node ID string does not contain NUL bytes");
        // Technically, string allocation may fail but `UA_NODEID_STRING_ALLOC` doesn't tell us that
        // when it happens. Instead, we end up with a well-defined node ID that has an empty string.
        let inner = unsafe { UA_NODEID_STRING_ALLOC(ns_index, string.as_ptr()) };
        debug_assert_eq!(inner.identifierType, UA_NodeIdType_UA_NODEIDTYPE_STRING);
        Self(inner)
    }
}

impl fmt::Debug for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = ua::String::default();

        let result = unsafe { UA_NodeId_print(self.as_ptr(), output.as_mut_ptr()) };
        if result != UA_STATUSCODE_GOOD {
            return f.write_str("NodeId");
        }

        match output.as_str() {
            Some(str) => f.write_str(str),
            None => f.write_str("NodeId"),
        }
    }
}
