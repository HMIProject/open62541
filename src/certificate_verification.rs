use crate::ua;

pub trait CustomCertificateVerification {
    fn verify_certificate(&self, certificate: &ua::ByteString) -> ua::StatusCode;

    fn verify_application_uri(
        &self,
        certificate: &ua::ByteString,
        application_uri: &ua::String,
    ) -> ua::StatusCode;
}
