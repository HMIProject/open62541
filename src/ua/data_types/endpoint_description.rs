use open62541_sys::UA_MessageSecurityMode;

use crate::ua;

crate::data_type!(EndpointDescription);
crate::member_accessors!(EndpointDescription {
    endpointUrl: &ua::String,
    securityPolicyUri: &ua::String,
    transportProfileUri: &ua::String,
    server: &ua::ApplicationDescription,
    serverCertificate: &ua::ByteString,
    #[enum(UA_MessageSecurityMode)]
    securityMode: ua::MessageSecurityMode,
    #[from_inner]
    securityLevel: ua::SecurityLevel
});
