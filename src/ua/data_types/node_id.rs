use std::ffi::CString;

use open62541_sys::{UA_NodeIdType, UA_NODEID_NUMERIC, UA_NODEID_STRING_ALLOC};

/// Typesafe wrapper for [`UA_NodeIdType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
#[allow(clippy::cast_possible_truncation)] // Verified by tests below.
pub enum NodeIdType {
    Numeric = UA_NodeIdType::UA_NODEIDTYPE_NUMERIC.0 as _,
    String = UA_NodeIdType::UA_NODEIDTYPE_STRING.0 as _,
    Guid = UA_NodeIdType::UA_NODEIDTYPE_GUID.0 as _,
    Opaque = UA_NodeIdType::UA_NODEIDTYPE_BYTESTRING.0 as _,
}

impl NodeIdType {
    /// Creates node ID type from raw value.
    ///
    /// # Panics
    ///
    /// Panics if the value is not a valid `UA_NodeIdType`.
    #[must_use]
    pub fn new(value: &UA_NodeIdType) -> Self {
        #[allow(non_upper_case_globals)]
        match value.0 {
            0 => Self::Numeric,
            3 => Self::String,
            4 => Self::Guid,
            5 => Self::Opaque,
            _ => panic!("invalid UA_NodeIdType: {value:?}"),
        }
    }

    /// Returns raw value of node ID type.
    ///
    /// All possible values of `UA_NodeIdType` fit into `u8`.
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        self as _
    }
}

crate::data_type!(NodeId, UA_NodeId, UA_TYPES_NODEID);

impl NodeId {
    /// Creates node ID for numeric identifier.
    #[must_use]
    pub fn numeric(ns_index: u16, numeric: u32) -> Self {
        let inner = unsafe { UA_NODEID_NUMERIC(ns_index, numeric) };
        debug_assert_eq!(
            inner.identifierType,
            UA_NodeIdType::UA_NODEIDTYPE_NUMERIC,
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
    pub fn string(ns_index: u16, string: &str) -> Self {
        let string = CString::new(string).expect("node ID string does not contain NUL bytes");

        // Technically, string allocation may fail but `UA_NODEID_STRING_ALLOC` doesn't tell us that
        // when it happens. Instead, we end up with a well-defined node ID that has an empty string.
        let inner = unsafe { UA_NODEID_STRING_ALLOC(ns_index, string.as_ptr()) };
        debug_assert_eq!(
            inner.identifierType,
            UA_NodeIdType::UA_NODEIDTYPE_STRING,
            "node ID is of string type"
        );

        // SAFETY: We have checked that we have this enum variant.
        let identifier = unsafe { inner.identifier.string.as_ref() };
        if !string.is_empty() && (identifier.data.is_null() || identifier.length == 0) {
            // We don't want to leak memory on top.
            debug_assert!(identifier.data.is_null());
            panic!("node ID string has been allocated");
        }

        Self(inner)
    }

    #[must_use]
    pub fn identifier_type(&self) -> NodeIdType {
        NodeIdType::new(&self.0.identifierType)
    }
}

#[cfg(test)]
mod tests {
    use open62541_sys::UA_NodeIdType;

    use super::*;

    #[test]
    fn node_type_id_numeric() {
        assert_eq!(
            UA_NodeIdType::UA_NODEIDTYPE_NUMERIC.0,
            NodeIdType::Numeric.as_u8() as _,
        );
        assert_eq!(
            NodeIdType::Numeric,
            NodeIdType::new(&UA_NodeIdType::UA_NODEIDTYPE_NUMERIC),
        );
    }

    #[test]
    fn node_type_id_string() {
        assert_eq!(
            UA_NodeIdType::UA_NODEIDTYPE_STRING.0,
            NodeIdType::String.as_u8() as _,
        );
        assert_eq!(
            NodeIdType::String,
            NodeIdType::new(&UA_NodeIdType::UA_NODEIDTYPE_STRING),
        );
    }

    #[test]
    fn node_type_id_guid() {
        assert_eq!(
            UA_NodeIdType::UA_NODEIDTYPE_GUID.0,
            NodeIdType::Guid.as_u8() as _,
        );
        assert_eq!(
            NodeIdType::Guid,
            NodeIdType::new(&UA_NodeIdType::UA_NODEIDTYPE_GUID),
        );
    }

    #[test]
    fn node_type_id_opaque() {
        assert_eq!(
            UA_NodeIdType::UA_NODEIDTYPE_BYTESTRING.0,
            NodeIdType::Opaque.as_u8() as _,
        );
        assert_eq!(
            NodeIdType::Opaque,
            NodeIdType::new(&UA_NodeIdType::UA_NODEIDTYPE_BYTESTRING),
        );
    }
}
