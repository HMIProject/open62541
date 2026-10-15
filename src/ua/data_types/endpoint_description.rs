use crate::ua;

crate::data_type!(EndpointDescription);
crate::member_accessors!(EndpointDescription {
    endpointUrl: &ua::String,
    securityPolicyUri: &ua::String,
    transportProfileUri: &ua::String,
    server: &ua::ApplicationDescription,
    serverCertificate: &ua::ByteString,
    #[enum]
    securityMode: ua::MessageSecurityMode,
    #[from_inner]
    securityLevel: ua::SecurityLevel
});
