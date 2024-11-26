use open62541_sys::UA_CertificateFormat;

/// Wrapper for [`UA_CertificateFormat`] from [`open62541_sys`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CertificateFormat(UA_CertificateFormat);

impl CertificateFormat {
    /// [DER] format.
    ///
    /// [DER]: https://en.wikipedia.org/wiki/X.690#DER_encoding
    pub const DER: Self = Self(UA_CertificateFormat::UA_CERTIFICATEFORMAT_DER);

    /// [PEM] format.
    ///
    /// [PEM]: https://en.wikipedia.org/wiki/Privacy-Enhanced_Mail
    pub const PEM: Self = Self(UA_CertificateFormat::UA_CERTIFICATEFORMAT_PEM);

    /// Gives up ownership and returns value.
    pub(crate) fn into_raw(self) -> UA_CertificateFormat {
        self.0
    }
}
