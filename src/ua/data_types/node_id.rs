use core::slice;
use std::ffi::CString;

use open62541_sys::{UA_Guid, UA_NodeIdType, UA_NODEID_NUMERIC, UA_NODEID_STRING_ALLOC};

use crate::ua;

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

    /// Identifier type
    #[must_use]
    pub fn identifier_type(&self) -> ua::NodeIdType {
        ua::NodeIdType::new(self.0.identifierType.clone())
    }

    /// Namespace index
    #[must_use]
    pub const fn namespace_index(&self) -> u16 {
        self.0.namespaceIndex
    }

    /// Numeric value
    ///
    /// Returns the numeric value if the type is numeric or `None` otherwise.
    #[must_use]
    pub fn numeric_value(&self) -> Option<u32> {
        if self.0.identifierType.0 != UA_NodeIdType::UA_NODEIDTYPE_NUMERIC.0 {
            return None;
        }
        // SAFETY: We have checked that we have this enum variant.
        let value = unsafe { *self.0.identifier.numeric.as_ref() };
        Some(value)
    }

    /// GUID value
    ///
    /// Returns the a reference to the value if the type is GUID or `None` otherwise.
    #[must_use]
    pub fn guid_value(&self) -> Option<&UA_Guid> {
        if self.0.identifierType.0 != UA_NodeIdType::UA_NODEIDTYPE_GUID.0 {
            return None;
        }
        // SAFETY: We have checked that we have this enum variant.
        let value = unsafe { self.0.identifier.guid.as_ref() };
        Some(value)
    }

    /// String value
    ///
    /// Returns the string value if the type is string or `None` otherwise.
    #[must_use]
    pub fn string_value(&self) -> Option<&str> {
        if self.0.identifierType.0 != UA_NodeIdType::UA_NODEIDTYPE_STRING.0 {
            return None;
        }
        // SAFETY: We have checked that we have this enum variant.
        let utf8 = unsafe {
            slice::from_raw_parts(
                self.0.identifier.string.as_ref().data,
                self.0.identifier.string.as_ref().length,
            )
        };
        let from_utf8 = core::str::from_utf8(utf8);
        debug_assert!(from_utf8.is_ok());
        from_utf8.ok()
    }

    /// Byte string value
    ///
    /// Returns the value if the type is byte string or `None` otherwise.
    #[must_use]
    pub fn byte_string_value(&self) -> Option<&[u8]> {
        if self.0.identifierType.0 != UA_NodeIdType::UA_NODEIDTYPE_BYTESTRING.0 {
            return None;
        }
        // SAFETY: We have checked that we have this enum variant.
        let bytes = unsafe {
            slice::from_raw_parts(
                self.0.identifier.byteString.as_ref().data,
                self.0.identifier.byteString.as_ref().length,
            )
        };
        Some(bytes)
    }
}
