use std::ptr::NonNull;

use open62541_sys::{UA_Client, UA_Client_delete, UA_Client_new};

/// Wrapper for [`UA_Client`] from [`open62541_sys`].
///
/// This owns the wrapped data type. When the wrapper is dropped, its inner value is cleaned up with
/// [`UA_Client_delete()`].
pub struct Client(NonNull<UA_Client>);

// SAFETY: We know that the underlying `UA_Client` allows access from different threads (at least as
// long as we do not call functions concurrently).
unsafe impl Send for Client {}

impl Client {
    #[allow(dead_code)]
    #[must_use]
    pub(crate) const fn as_ptr(&self) -> *const UA_Client {
        self.0.as_ptr()
    }

    #[allow(dead_code)]
    #[must_use]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut UA_Client {
        self.0.as_ptr()
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        // `UA_Client_delete()` matches `UA_Client_new()`.
        unsafe { UA_Client_delete(self.as_mut_ptr()) }
    }
}

impl Default for Client {
    /// Creates wrapper initialized with defaults.
    fn default() -> Self {
        // `UA_Client_new()` matches `UA_Client_delete()`.
        let inner = NonNull::new(unsafe { UA_Client_new() }).expect("create new UA_Client");
        Self(inner)
    }
}
