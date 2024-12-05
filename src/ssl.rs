use std::{cell::RefCell, fmt, ptr, rc::Rc};

use open62541_sys::UA_CreateCertificate;

use crate::{ua, ClientBuilder, CustomCertificateVerification, DataType, Error, Result};

/// Certificate in [DER] or [PEM] format.
///
/// [DER]: https://en.wikipedia.org/wiki/X.690#DER_encoding
/// [PEM]: https://en.wikipedia.org/wiki/Privacy-Enhanced_Mail
#[derive(Debug, Clone)]
pub struct Certificate(ua::ByteString);

impl Certificate {
    pub(crate) fn from_byte_string(byte_string: ua::ByteString) -> Option<Self> {
        (!byte_string.is_invalid()).then(|| Self(byte_string))
    }

    pub(crate) unsafe fn from_string_unchecked(string: ua::String) -> Self {
        Self::from_byte_string(string.into_byte_string()).expect("certificate should be set")
    }

    /// Wraps certificate data.
    ///
    /// This does not validate the data. When passing the instance to another method, that method
    /// may still fail if the certificate is not valid.
    #[must_use]
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self(ua::ByteString::new(bytes))
    }

    /// Gets certificate data.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        // SAFETY: We always initialize inner value.
        unsafe { self.0.as_bytes_unchecked() }
    }

    pub(crate) const fn as_byte_string(&self) -> &ua::ByteString {
        &self.0
    }
}

/// Private key in [DER] or [PEM] format.
///
/// [DER]: https://en.wikipedia.org/wiki/X.690#DER_encoding
/// [PEM]: https://en.wikipedia.org/wiki/Privacy-Enhanced_Mail
#[derive(Clone)]
pub struct PrivateKey(ua::ByteString);

impl PrivateKey {
    pub(crate) fn from_byte_string(byte_string: ua::ByteString) -> Option<Self> {
        (!byte_string.is_invalid()).then(|| Self(byte_string))
    }

    pub(crate) unsafe fn from_string_unchecked(string: ua::String) -> Self {
        Self::from_byte_string(string.into_byte_string()).expect("private key should be set")
    }

    /// Wraps private key data.
    ///
    /// This does not validate the data. When passing the instance to another method, that method
    /// may still fail if the private key is not valid.
    #[must_use]
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self(ua::ByteString::new(bytes))
    }

    /// Gets certificate data.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        // SAFETY: We always initialize inner value.
        unsafe { self.0.as_bytes_unchecked() }
    }

    pub(crate) const fn as_byte_string(&self) -> &ua::ByteString {
        &self.0
    }
}

impl fmt::Debug for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Omit display of private key to not leak secrets.
        f.debug_tuple("PrivateKey").finish()
    }
}

/// Creates SSL certificate.
///
/// This creates an SSL certificate and accompanying private key, to be used in (but not limited to)
/// [`ClientBuilder`] or [`ServerBuilder`].
///
/// The argument `subject` should contain the necessary subject instructions, e.g.
///
/// - `C=DE`
/// - `O=SampleOrganization`
/// - `CN=Open62541Server@localhost`
///
/// The argument `subject_alt_names` should contain the subject alternative names as required, e.g.
///
/// - `DNS:localhost`
/// - `URI:urn:open62541.server.application` (match application URI in application description)
///
/// The argument `params` may overwrite the default values of these parameters:
///
/// - `key-size-bits` (`UInt16`)
/// - `expires-in-days` (`UInt16`)
///
/// # Errors
///
/// This fails when certificate cannot be generated (invalid arguments or internal error).
///
/// [`ClientBuilder`]: crate::ClientBuilder
/// [`ServerBuilder`]: crate::ServerBuilder
pub fn create_certificate(
    subject: &ua::Array<ua::String>,
    subject_alt_name: &ua::Array<ua::String>,
    cert_format: &ua::CertificateFormat,
    params: Option<&ua::KeyValueMap>,
) -> Result<(Certificate, PrivateKey)> {
    // Create logger that forwards to Rust `log`. It is only used for the function call below and it
    // will be cleaned up at the end of the function.
    let mut logger = ua::Logger::rust_log();

    // These are out arguments for the function call and need not be initialized.
    let mut private_key = ua::String::invalid();
    let mut certificate = ua::String::invalid();

    let status_code = ua::StatusCode::new(unsafe {
        // SAFETY: The arrays live until `UA_CreateCertificate()` returns and that function does not
        // take ownership.
        let (subject_size, subject_ptr) = subject.as_raw_parts();
        let (subject_alt_name_size, subject_alt_name_ptr) = subject_alt_name.as_raw_parts();

        UA_CreateCertificate(
            // SAFETY: The function does not take ownership and does not use the logger afterwards.
            logger.as_mut_ptr(),
            subject_ptr,
            subject_size,
            subject_alt_name_ptr,
            subject_alt_name_size,
            ua::CertificateFormat::to_raw_copy(cert_format),
            // SAFETY: Function only reads from value, so casting to non-const pointer is safe here.
            params.map_or_else(ptr::null_mut, |params| params.as_ptr().cast_mut()),
            // SAFETY: The function does not become the owner of these out arguments but it puts the
            // results into them.
            private_key.as_mut_ptr(),
            certificate.as_mut_ptr(),
        )
    });
    Error::verify_good(&status_code)?;

    // SAFETY: The function is expected to return valid strings in its output arguments.
    let certificate = unsafe { Certificate::from_string_unchecked(certificate) };
    let private_key = unsafe { PrivateKey::from_string_unchecked(private_key) };

    Ok((certificate, private_key))
}

/// Connects to remote server and fetches certificate
///
/// # Errors
///
/// This fails when the connection cannot be established at all or the server does not offer secure
/// communication.
pub fn fetch_server_certificate(
    local_certificate: &Certificate,
    private_key: &PrivateKey,
    endpoint_url: &str,
) -> Result<Certificate> {
    let (fetch_server_certificate_verification, certificate) =
        FetchServerCertificateVerification::new();

    let certificate_verification =
        ua::CertificateVerification::custom(fetch_server_certificate_verification);

    let _unused = ClientBuilder::default_encryption(local_certificate, private_key)?
        .certificate_verification(certificate_verification)
        .connect(endpoint_url)?;

    certificate
        .take()
        .ok_or(Error::internal("did not receive certificate"))
}

#[derive(Debug)]
struct FetchServerCertificateVerification {
    certificate: Rc<RefCell<Option<Certificate>>>,
}

impl FetchServerCertificateVerification {
    fn new() -> (Self, Rc<RefCell<Option<Certificate>>>) {
        let certificate = Rc::new(RefCell::new(None));

        (
            Self {
                certificate: Rc::clone(&certificate),
            },
            certificate,
        )
    }
}

impl CustomCertificateVerification for FetchServerCertificateVerification {
    fn verify_certificate(&self, certificate: &ua::ByteString) -> ua::StatusCode {
        self.certificate
            .replace(Certificate::from_byte_string(certificate.clone()));

        ua::StatusCode::GOOD
    }

    fn verify_application_uri(
        &self,
        _certificate: &ua::ByteString,
        _application_uri: &ua::String,
    ) -> ua::StatusCode {
        ua::StatusCode::GOOD
    }
}
