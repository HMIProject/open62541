use crate::{DataType, ua};

crate::data_type!(EndpointDescription);

impl EndpointDescription {
    #[must_use]
    pub fn endpoint_url(&self) -> &ua::String {
        ua::String::raw_ref(&self.0.endpointUrl)
    }

    #[must_use]
    pub fn server(&self) -> &ua::ApplicationDescription {
        ua::ApplicationDescription::raw_ref(&self.0.server)
    }

    #[must_use]
    pub fn server_certificate(&self) -> &ua::ByteString {
        ua::ByteString::raw_ref(&self.0.serverCertificate)
    }

    #[must_use]
    pub fn security_mode(&self) -> &ua::MessageSecurityMode {
        ua::MessageSecurityMode::raw_ref(&self.0.securityMode)
    }

    #[must_use]
    pub fn security_policy_uri(&self) -> &ua::String {
        ua::String::raw_ref(&self.0.securityPolicyUri)
    }

    #[must_use]
    pub fn transport_profile_uri(&self) -> &ua::String {
        ua::String::raw_ref(&self.0.transportProfileUri)
    }

    #[must_use]
    pub const fn security_level(&self) -> ua::SecurityLevel {
        ua::SecurityLevel::new(self.0.securityLevel)
    }
}
