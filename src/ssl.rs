use std::ptr;

use open62541_sys::UA_CreateCertificate;

use crate::{ua, DataType, Error, Result};

#[derive(Debug)]
pub struct Certificate {
    /// Private key, always in DER format.
    pub private_key: ua::ByteString,

    /// Certificate, format as given by [`ua::CertificateFormat`].
    pub certificate: ua::ByteString,
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
/// - `key-size-bits` (UInt16)
/// - `expires-in-days` (UInt16)
///
/// [`ClientBuilder`]: crate::ClientBuilder
/// [`ServerBuilder`]: crate::ServerBuilder
pub fn create_certificate(
    subject: &ua::Array<ua::String>,
    subject_alt_name: &ua::Array<ua::String>,
    cert_format: &ua::CertificateFormat,
    params: Option<&ua::KeyValueMap>,
) -> Result<Certificate> {
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

    Ok(Certificate {
        private_key: private_key.into_byte_string(),
        certificate: certificate.into_byte_string(),
    })
}
