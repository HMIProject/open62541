use std::ptr::NonNull;

use open62541_sys::{UA_Server, UA_Server_delete, UA_Server_new};

use crate::{ua, Error};

/// Wrapper for [`UA_Server`] from [`open62541_sys`].
///
/// This owns the wrapped data type. When the wrapper is dropped, its inner value is cleaned up with
/// [`UA_Server_delete()`].
pub struct Server(NonNull<UA_Server>);

impl Server {
    /// Creates server.
    #[must_use]
    pub(crate) fn new() -> Self {
        let inner = unsafe { UA_Server_new() };
        // PANIC: The only possible errors here are out-of-memory.
        let inner = NonNull::new(inner).expect("create UA_Server");
        Self(inner)
    }

    /// Returns mutable pointer to value.
    ///
    /// # Safety
    ///
    /// The value is owned by `Self`. Ownership must not be given away, in whole or in parts. This
    /// may happen when `open62541` functions are called that take ownership of values by pointer.
    #[must_use]
    pub(crate) const unsafe fn as_mut_ptr(&self) -> *mut UA_Server {
        self.0.as_ptr()
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        log::debug!("Deleting server");

        let status_code = ua::StatusCode::new(unsafe {
            // SAFETY: We retain ownership of `self`.
            UA_Server_delete(self.as_mut_ptr())
        });
        if let Err(error) = Error::verify_good(&status_code) {
            log::warn!("Error while dropping server: {error}");
        }
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}
