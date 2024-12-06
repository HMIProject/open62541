crate::data_type!(MessageSecurityMode, UInt32);

crate::enum_variants!(
    MessageSecurityMode,
    UA_MessageSecurityMode,
    [INVALID, NONE, SIGN, SIGNANDENCRYPT]
);
