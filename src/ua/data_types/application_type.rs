crate::data_type!(ApplicationType, UInt32);

crate::enum_variants!(
    ApplicationType,
    UA_ApplicationType,
    [SERVER, CLIENT, CLIENTANDSERVER, DISCOVERYSERVER],
);
