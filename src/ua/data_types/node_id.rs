use std::ffi::CString;

use open62541_sys::{
    UA_NodeIdType_UA_NODEIDTYPE_NUMERIC, UA_NodeIdType_UA_NODEIDTYPE_STRING, UA_NODEID_NUMERIC,
    UA_NODEID_STRING_ALLOC,
};

crate::data_type!(NodeId, UA_NodeId, UA_TYPES_NODEID);

impl NodeId {
    /// Creates node ID for numeric identifier.
    #[must_use]
    pub fn new_numeric(ns_index: u16, numeric: u32) -> Self {
        let inner = unsafe { UA_NODEID_NUMERIC(ns_index, numeric) };
        debug_assert_eq!(
            inner.identifierType, UA_NodeIdType_UA_NODEIDTYPE_NUMERIC,
            "node ID is of numeric type"
        );

        Self(inner)
    }

    /// Creates node ID for string identifier.
    ///
    /// # Panics
    ///
    /// The string identifier must be a valid C string, i.e. it must not contain any NUL bytes. Also
    /// there must be enough memory available to allocate string.
    #[must_use]
    pub fn new_string(ns_index: u16, string: &str) -> Self {
        let string = CString::new(string).expect("node ID string does not contain NUL bytes");

        // Technically, string allocation may fail but `UA_NODEID_STRING_ALLOC` doesn't tell us that
        // when it happens. Instead, we end up with a well-defined node ID that has an empty string.
        let inner = unsafe { UA_NODEID_STRING_ALLOC(ns_index, string.as_ptr()) };
        debug_assert_eq!(
            inner.identifierType, UA_NodeIdType_UA_NODEIDTYPE_STRING,
            "node ID is of string type"
        );

        // SAFETY: We have checked that we have this enum variant.
        let identifier = unsafe { inner.identifier.string };
        if !string.is_empty() && (identifier.data.is_null() || identifier.length == 0) {
            // We don't want to leak memory on top.
            debug_assert!(identifier.data.is_null());
            panic!("node ID string has been allocated");
        }

        Self(inner)
    }
}
