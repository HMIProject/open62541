use std::ptr;

use open62541_sys::UA_CreateCertificate;

use crate::{ua, DataType, Error, Result};

#[derive(Debug)]
pub struct Certificate {
    pub private_key: ua::ByteString,
    pub certificate: ua::ByteString,
}

pub fn create_certificate(
    subject: impl IntoIterator<Item = ua::String>,
    subject_alt_name: impl IntoIterator<Item = ua::String>,
    cert_format: ua::CertificateFormat,
) -> Result<Certificate> {
    // Create logger that forwards to Rust `log`. It is only used for the function call below and it
    // will be cleaned up at the end of the function.
    let mut logger = ua::Logger::rust_log();

    let subject = ua::Array::from_iter(subject.into_iter());
    let (subject_size, subject_ptr) = unsafe { subject.as_raw_parts() };

    let subject_alt_name = ua::Array::from_iter(subject_alt_name.into_iter());
    let (subject_alt_name_size, subject_alt_name_ptr) = unsafe { subject_alt_name.as_raw_parts() };

    // These are out arguments for the function call and need not be initialized.
    let mut private_key = ua::String::invalid();
    let mut certificate = ua::String::invalid();

    let status_code = ua::StatusCode::new(unsafe {
        UA_CreateCertificate(
            // SAFETY: The function does not take ownership and does not use the logger afterwards.
            logger.as_mut_ptr(),
            subject_ptr,
            subject_size,
            subject_alt_name_ptr,
            subject_alt_name_size,
            // SAFETY: The underlying value is trivial (`u32`), so the ownership is irrelevant here.
            cert_format.into_raw(),
            ptr::null_mut(),
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
