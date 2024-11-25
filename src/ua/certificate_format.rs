use open62541_sys::UA_CertificateFormat;

/// Wrapper for [`UA_CertificateFormat`] from [`open62541_sys`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CertificateFormat(UA_CertificateFormat);

impl CertificateFormat {
    pub const DER: Self = Self(UA_CertificateFormat::UA_CERTIFICATEFORMAT_DER);
    pub const PEM: Self = Self(UA_CertificateFormat::UA_CERTIFICATEFORMAT_PEM);

    /// Gives up ownership and returns value.
    pub(crate) fn into_raw(self) -> UA_CertificateFormat {
        self.0
    }
}
