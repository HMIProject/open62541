use std::fmt;

use open62541_sys::UA_NodeIdType;

/// Wrapper for node ID types from [`open62541_sys`].
#[derive(Clone, Debug)]
pub struct NodeIdType(UA_NodeIdType);

impl NodeIdType {
    #[must_use]
    pub const fn numeric() -> Self {
        Self(UA_NodeIdType::UA_NODEIDTYPE_NUMERIC)
    }

    #[must_use]
    pub const fn string() -> Self {
        Self(UA_NodeIdType::UA_NODEIDTYPE_STRING)
    }

    #[must_use]
    pub const fn guid() -> Self {
        Self(UA_NodeIdType::UA_NODEIDTYPE_GUID)
    }

    #[must_use]
    pub const fn byte_string() -> Self {
        Self(UA_NodeIdType::UA_NODEIDTYPE_BYTESTRING)
    }

    /// Creates wrapper by taking ownership of `src`.
    #[allow(dead_code)]
    #[must_use]
    pub(crate) const fn new(src: UA_NodeIdType) -> Self {
        Self(src)
    }

    /// Gives up ownership and returns inner value.
    #[allow(dead_code)]
    #[must_use]
    pub(crate) const fn into_inner(self) -> UA_NodeIdType {
        self.0
    }
}

impl fmt::Display for NodeIdType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match &self.0 {
            &UA_NodeIdType::UA_NODEIDTYPE_NUMERIC => "NUMERIC",
            &UA_NodeIdType::UA_NODEIDTYPE_STRING => "STRING",
            &UA_NodeIdType::UA_NODEIDTYPE_GUID => "GUID",
            &UA_NodeIdType::UA_NODEIDTYPE_BYTESTRING => "BYTESTRING",
            _ => "?",
        };
        f.write_str(str)
    }
}
