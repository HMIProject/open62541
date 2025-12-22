use std::{
    mem::{self, ManuallyDrop, MaybeUninit},
    ptr,
};

use derive_more::Debug;
#[cfg(feature = "mbedtls")]
use open62541_sys::UA_CertificateGroup_Memorystore;
use open62541_sys::{UA_CertificateGroup, UA_CertificateGroup_AcceptAll};

#[cfg(feature = "mbedtls")]
use crate::{DataType as _, ua};

/// Wrapper for [`UA_CertificateGroup`] from [`open62541_sys`].
#[derive(Debug)]
pub struct CertificateVerification(#[debug(skip)] UA_CertificateGroup);

impl CertificateVerification {
    /// Creates certificate verification with all checks disabled.
    ///
    /// Note that this disables certificate verification entirely. Use only when the other end can
    /// be identified in some other way, or identity is not relevant.
    #[must_use]
    pub fn accept_all() -> Self {
        let mut certificate_verification = Self::init();
        // SAFETY: Certificate verification is uninitialized, but that is expected.
        unsafe {
            UA_CertificateGroup_AcceptAll(certificate_verification.as_mut_ptr());
        }
        certificate_verification
    }

    #[cfg(feature = "mbedtls")]
    #[must_use]
    pub fn memory_store(
        certificate_group_id: &ua::NodeId,
        trust_list: Option<&ua::TrustListDataType>,
    ) -> Self {
        let mut certificate_verification = Self::init();
        // SAFETY: Certificate verification is uninitialized, but that is expected.
        unsafe {
            UA_CertificateGroup_Memorystore(
                certificate_verification.as_mut_ptr(),
                // SAFETY: Argument is only read, not modified or returned.
                certificate_group_id.as_ptr().cast_mut(),
                trust_list.map_or(ptr::null(), |trust_list| trust_list.as_ptr()),
                // FIXME: Initialize logger.
                ptr::null(),
                // TODO: Allow parameters. Valid parameters are:
                // - "max-trust-listsize"
                // - "max-rejected-listsize"
                ptr::null(),
            );
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
    pub(crate) const unsafe fn from_raw(src: UA_CertificateGroup) -> Self {
        Self(src)
    }

    /// Gives up ownership and returns value.
    ///
    /// The returned value must be re-wrapped with [`from_raw()`], cleared manually, or copied into
    /// an owning value (like [`UA_Client`]) to free internal allocations and not leak memory.
    ///
    /// [`from_raw()`]: Self::from_raw
    /// [`UA_Client`]: open62541_sys::UA_Client
    #[expect(clippy::allow_attributes, reason = "non-static condition")]
    #[allow(clippy::missing_const_for_fn, reason = "unsupported before Rust 1.87")]
    #[must_use]
    pub(crate) fn into_raw(self) -> UA_CertificateGroup {
        // Use `ManuallyDrop` to avoid double-free even when added code might cause panic. See
        // documentation of `mem::forget()` for details.
        let this = ManuallyDrop::new(self);
        // SAFETY: Aliasing memory temporarily is safe because destructor will not be called.
        unsafe { ptr::read(&raw const this.0) }
    }

    /// Creates wrapper initialized with defaults.
    ///
    /// This initializes the value and makes all attributes well-defined. Additional attributes may
    /// need to be initialized for the value to be actually useful afterwards.
    pub(crate) const fn init() -> Self {
        let inner = MaybeUninit::<UA_CertificateGroup>::zeroed();
        // SAFETY: Zero-initialized memory is a valid certificate verification.
        let inner = unsafe { inner.assume_init() };
        // SAFETY: We pass a value without pointers to it into `Self`.
        unsafe { Self::from_raw(inner) }
    }

    /// Moves value into `dst`, giving up ownership.
    ///
    /// Existing data in `dst` is cleared before moving the value; it is safe to use this operation
    /// on already initialized target values.
    ///
    /// The logging reference will be transferred from the old to the new certificate verification.
    ///
    /// After this, it is the responsibility of `dst` to eventually clean up the data.
    pub(crate) fn move_into_raw(self, dst: &mut UA_CertificateGroup) {
        // Move certificate verification into target, transferring ownership.
        let orig = mem::replace(dst, self.into_raw());
        // Take ownership of previously set certificate verification in order to drop it.
        let mut orig = unsafe { Self::from_raw(orig) };
        // Before dropping, transfer previously set logging to new certificate verification. We do
        // this because certificate verifications do not own the logging reference. For instance,
        // after creating a new config, the config's owned logger is copied (!) here. Refer to
        // comments in `ClientConfig::new()` for more info.
        mem::swap(&mut dst.logging, &mut unsafe { orig.as_mut() }.logging);
    }

    /// Returns exclusive reference to value.
    ///
    /// # Safety
    ///
    /// The value is owned by `Self`. Ownership must not be given away, in whole or in parts. This
    /// may happen when `open62541` functions are called that take ownership of values by pointer.
    #[must_use]
    #[expect(clippy::allow_attributes, reason = "non-static condition")]
    #[allow(clippy::missing_const_for_fn, reason = "unsupported before Rust 1.87")]
    pub(crate) unsafe fn as_mut(&mut self) -> &mut UA_CertificateGroup {
        &mut self.0
    }

    /// Returns mutable pointer to value.
    ///
    /// # Safety
    ///
    /// The value is owned by `Self`. Ownership must not be given away, in whole or in parts. This
    /// may happen when `open62541` functions are called that take ownership of values by pointer.
    #[must_use]
    #[expect(clippy::allow_attributes, reason = "non-static condition")]
    #[allow(clippy::missing_const_for_fn, reason = "unsupported before Rust 1.87")]
    pub(crate) unsafe fn as_mut_ptr(&mut self) -> *mut UA_CertificateGroup {
        &raw mut self.0
    }
}

impl Drop for CertificateVerification {
    fn drop(&mut self) {
        if let Some(clear) = self.0.clear {
            unsafe {
                clear(&raw mut self.0);
            }
        }
    }
}
