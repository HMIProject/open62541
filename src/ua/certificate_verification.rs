use std::{
    mem::{self, MaybeUninit},
    ptr,
};

use open62541_sys::{UA_CertificateVerification, UA_CertificateVerification_AcceptAll};

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
