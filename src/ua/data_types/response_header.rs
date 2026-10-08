use crate::ua;

crate::data_type!(ResponseHeader);
crate::member_accessors!(ResponseHeader {
    additionalHeader: &ua::ExtensionObject,
    requestHandle: u32,
    timestamp: i64,
    serviceDiagnostics: &ua::DiagnosticInfo,
    #[from_inner]
    serviceResult: ua::StatusCode,
    stringTable: [ua::String]
});
