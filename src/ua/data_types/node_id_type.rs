use open62541_sys::UA_NodeIdType;

crate::data_type!(NodeIdType, UInt32);

crate::enum_variants!(
    NodeIdType,
    UA_NodeIdType,
    [NUMERIC, STRING, GUID, BYTESTRING],
);

impl NodeIdType {
    #[deprecated(note = "use `Self::NUMERIC` instead")]
    #[must_use]
    pub const fn numeric() -> Self {
        Self(UA_NodeIdType::UA_NODEIDTYPE_NUMERIC)
    }

    #[deprecated(note = "use `Self::STRING` instead")]
    #[must_use]
    pub const fn string() -> Self {
        Self(UA_NodeIdType::UA_NODEIDTYPE_STRING)
    }

    #[deprecated(note = "use `Self::GUID` instead")]
    #[must_use]
    pub const fn guid() -> Self {
        Self(UA_NodeIdType::UA_NODEIDTYPE_GUID)
    }

    #[deprecated(note = "use `Self::BYTESTRING` instead")]
    #[must_use]
    pub const fn byte_string() -> Self {
        Self(UA_NodeIdType::UA_NODEIDTYPE_BYTESTRING)
    }
}
