use std::ptr::{self, NonNull};

use open62541_sys::{UA_Server, UA_Server_delete, UA_Server_newWithConfig};

use crate::{ua, Error};

/// Wrapper for [`UA_Server`] from [`open62541_sys`].
///
/// This owns the wrapped data type. When the wrapper is dropped, its inner value is cleaned up with
/// [`UA_Server_delete()`].
pub struct Server(NonNull<UA_Server>);

// SAFETY: We know that the underlying `UA_Server` allows access from different threads, i.e. it may
// be dropped in a different thread from where it was created.
unsafe impl Send for Server {}

// SAFETY: The underlying `UA_Server` can be used from different threads concurrently, at least with
// _most_ methods (those marked `UA_THREADSAFE` and/or with explicit mutex locks inside).
unsafe impl Sync for Server {}

impl Server {
    /// Creates server from server config.
    ///
    /// This consumes the config object and makes the server the owner of all contained data therein
    /// (e.g. logging configuration and logger instance).
    pub(crate) fn new_with_config(config: ua::ServerConfig) -> Self {
        let mut config = config.into_raw();
        let inner = unsafe { UA_Server_newWithConfig(ptr::addr_of_mut!(config)) };
        // PANIC: The only possible errors here are out-of-memory.
        let inner = NonNull::new(inner).expect("create UA_Server");
        Self(inner)
    }

    pub(crate) fn from_raw(raw_server: *mut UA_Server) -> Self {
        // PANIC: The only possible errors here are out-of-memory.
        let inner = NonNull::new(raw_server).expect("create UA_Server");
        Self(inner)
    }

    /// Returns const pointer to value.
    ///
    /// # Safety
    ///
    /// The value is owned by `Self`. Ownership must not be given away, in whole or in parts. This
    /// may happen when `open62541` functions are called that take ownership of values by pointer.
    #[allow(dead_code)] // This is unused for now.
    #[must_use]
    pub(crate) const unsafe fn as_ptr(&self) -> *const UA_Server {
        self.0.as_ptr()
    }

    /// Returns mutable pointer to value.
    ///
    /// # Safety
    ///
    /// The value is owned by `Self`. Ownership must not be given away, in whole or in parts. This
    /// may happen when `open62541` functions are called that take ownership of values by pointer.
    #[must_use]
    pub(crate) unsafe fn as_mut_ptr(&mut self) -> *mut UA_Server {
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
