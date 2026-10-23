crate::data_type!(UserTokenType, UInt32);
crate::enum_variants!(
    UserTokenType,
    UA_UserTokenType,
    [ANONYMOUS, USERNAME, CERTIFICATE, ISSUEDTOKEN,]
);
