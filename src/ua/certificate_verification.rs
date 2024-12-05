use std::{
    mem::{self, MaybeUninit},
    ptr,
};

use open62541_sys::{
    UA_ByteString, UA_CertificateVerification, UA_CertificateVerification_AcceptAll, UA_StatusCode,
    UA_String,
};

use crate::{ua, CustomCertificateVerification, DataType, Userdata};

/// Wrapper for [`UA_CertificateVerification`] from [`open62541_sys`].
#[derive(Debug)]
pub struct CertificateVerification(UA_CertificateVerification);

impl CertificateVerification {
    /// Certificate verification with all checks disabled.
    ///
    /// Note that this disables certificate verification entirely. Use only when the other end can
    /// be identified in some other way, or identity is not relevant.
    #[must_use]
    pub fn accept_all() -> Self {
        let mut certificate_verification = Self::init();
        // SAFETY: Certificate verification is null, but that is valid.
        unsafe {
            UA_CertificateVerification_AcceptAll(certificate_verification.as_mut_ptr());
        }
        certificate_verification
    }

    pub fn custom(certificate_verification: impl CustomCertificateVerification + 'static) -> Self {
        type Ud = Userdata<Box<dyn CustomCertificateVerification>>;

        unsafe extern "C" fn verify_certificate_c(
            cv: *const UA_CertificateVerification,
            certificate: *const UA_ByteString,
        ) -> UA_StatusCode {
            // SAFETY: Reference is used only for the remainder of this function.
            let certificate = ua::ByteString::raw_ref(unsafe {
                certificate.as_ref().expect("certificate should be set")
            });

            // SAFETY: We use the user data only when it is still alive.
            let certificate_verification = unsafe { Ud::peek_at((*cv).context) };
            let status_code = certificate_verification.verify_certificate(certificate);
            status_code.into_raw()
        }

        unsafe extern "C" fn verify_application_uri_c(
            cv: *const UA_CertificateVerification,
            certificate: *const UA_ByteString,
            application_uri: *const UA_String,
        ) -> UA_StatusCode {
            // SAFETY: References are used only for the remainder of this function.
            let certificate = ua::ByteString::raw_ref(unsafe {
                certificate.as_ref().expect("certificate should be set")
            });
            let application_uri = ua::String::raw_ref(unsafe {
                application_uri
                    .as_ref()
                    .expect("application URI should be set")
            });

            // SAFETY: We use the user data only when it is still alive.
            let certificate_verification = unsafe { Ud::peek_at((*cv).context) };
            let status_code =
                certificate_verification.verify_application_uri(certificate, application_uri);
            status_code.into_raw()
        }

        unsafe extern "C" fn clear_c(cv: *mut UA_CertificateVerification) {
            // Reclaim ownership of certificate verification and drop it.
            // SAFETY: We use the user data only when it is still alive.
            let _unused = unsafe { Ud::consume((*cv).context) };
        }

        let inner = UA_CertificateVerification {
            context: Ud::prepare(Box::new(certificate_verification)),
            verifyCertificate: Some(verify_certificate_c),
            verifyApplicationURI: Some(verify_application_uri_c),
            getExpirationDate: None,
            getSubjectName: None,
            clear: Some(clear_c),
            logging: ptr::null_mut(),
        };

        unsafe { Self::from_raw(inner) }
    }

    /// Creates wrapper by taking ownership of value.
    ///
    /// When `Self` is dropped, allocations held by the inner type are cleaned up.
    ///
    /// # Safety
    ///
    /// Ownership of the value passes to `Self`. This must only be used for values that are not
    /// contained within other values that may be dropped.
    #[must_use]
    pub(crate) const unsafe fn from_raw(src: UA_CertificateVerification) -> Self {
        Self(src)
    }

    /// Gives up ownership and returns value.
    ///
    /// The returned value must be re-wrapped with [`from_raw()`], cleared manually, or copied into
    /// an owning value (like [`UA_Client`]) to free internal allocations and not leak memory.
    ///
    /// [`from_raw()`]: Self::from_raw
    /// [`UA_Client`]: open62541_sys::UA_Client
    #[must_use]
    pub(crate) const fn into_raw(self) -> UA_CertificateVerification {
        // SAFETY: The inner object is valid, we do not drop it.
        let inner = unsafe { ptr::read(&self.0) };
        // Do not call `drop()`. Value now lives in `inner`.
        mem::forget(self);
        inner
    }

    /// Creates wrapper initialized with defaults.
    ///
    /// This initializes the value and makes all attributes well-defined. Additional attributes may
    /// need to be initialized for the value to be actually useful afterwards.
    pub(crate) const fn init() -> Self {
        let inner = MaybeUninit::<UA_CertificateVerification>::zeroed();
        // SAFETY: Zero-initialized memory is a valid certificate verification.
        let inner = unsafe { inner.assume_init() };
        // SAFETY: We pass a value without pointers to it into `Self`.
        unsafe { Self::from_raw(inner) }
    }

    /// Returns exclusive reference to value.
    ///
    /// # Safety
    ///
    /// The value is owned by `Self`. Ownership must not be given away, in whole or in parts. This
    /// may happen when `open62541` functions are called that take ownership of values by pointer.
    #[allow(dead_code)] // This is unused for now.
    #[must_use]
    pub(crate) unsafe fn as_mut(&mut self) -> &mut UA_CertificateVerification {
        &mut self.0
    }

    /// Returns mutable pointer to value.
    ///
    /// # Safety
    ///
    /// The value is owned by `Self`. Ownership must not be given away, in whole or in parts. This
    /// may happen when `open62541` functions are called that take ownership of values by pointer.
    #[must_use]
    pub(crate) unsafe fn as_mut_ptr(&mut self) -> *mut UA_CertificateVerification {
        ptr::addr_of_mut!(self.0)
    }
}

impl Drop for CertificateVerification {
    fn drop(&mut self) {
        if let Some(clear) = self.0.clear {
            unsafe {
                clear(ptr::addr_of_mut!(self.0));
            }
        }
    }
}
