use open62541_sys::UA_NodeIdType;

crate::data_type!(NodeIdType, UInt32);

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
}
