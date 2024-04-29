crate::data_type!(NodeIdType, UInt32);

crate::enum_variants!(
    NodeIdType,
    UA_NodeIdType,
    [NUMERIC, STRING, GUID, BYTESTRING],
);
