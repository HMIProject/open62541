mod rust_log;

use std::{mem, ptr::NonNull};

use open62541_sys::UA_Logger;

/// Wrapper for [`UA_Logger`] from [`open62541_sys`].
#[derive(Debug)]
pub(crate) struct Logger(NonNull<UA_Logger>);

impl Logger {
    /// Creates logger that forwards to the `log` crate.
    pub(crate) fn rust_log() -> Self {
        rust_log::logger()
    }

    /// Creates wrapper by taking ownership of value.
    ///
    /// When `Self` is dropped, allocations held by the inner type are freed.
    ///
    /// # Safety
    ///
    /// Ownership of the value passes to `Self`. This must only be used for values that are not
    /// contained within other values that may be dropped (such as attributes in other data types).
    ///
    /// # Panics
    ///
    /// The given pointer must be valid.
    pub(crate) unsafe fn from_raw(logger: *mut UA_Logger) -> Self {
        Self(NonNull::new(logger).expect("pointer must be non-null"))
    }

    /// Gives up ownership and returns value.
    pub(crate) const fn into_raw(self) -> *mut UA_Logger {
        let logger = self.0.as_ptr();
        // Make sure that `drop()` is not called anymore.
        mem::forget(self);
        logger
    }

    /// Returns mutable pointer to value.
    ///
    /// # Safety
    ///
    /// The value is owned by `Self`. Ownership must not be given away, in whole or in parts. This
    /// may happen when `open62541` functions are called that take ownership of values by pointer.
    #[allow(dead_code)] // --features mbedtls
    pub(crate) unsafe fn as_mut_ptr(&mut self) -> *mut UA_Logger {
        self.0.as_ptr()
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        // Let logger clean itself up when `clear()` callback has been set.
        if let Some(clear) = unsafe { self.0.as_ref() }.clear {
            let logger = self.0.as_ptr();
            unsafe { clear(logger) };
        }
    }
}
