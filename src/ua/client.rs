use std::ptr::NonNull;

use open62541_sys::{
    UA_Client, UA_Client_delete, UA_Client_disconnect, UA_Client_getState, UA_Client_new,
};

use crate::{ua, DataType as _, Error};

/// Combined state for [`Client`] and [`AsyncClient`].
///
/// [`AsyncClient`]: crate::AsyncClient
#[allow(clippy::module_name_repetitions)]
pub struct ClientState {
    pub channel_state: ua::SecureChannelState,
    pub session_state: ua::SessionState,
    /// The `connect_status` is set if the client connection (including reconnects) has failed and
    /// the client has to "give up". If the `connect_status` is not set, the client still has hope
    /// to connect or recover.
    pub connect_status: ua::StatusCode,
}

/// Wrapper for [`UA_Client`] from [`open62541_sys`].
///
/// This owns the wrapped data type. When the wrapper is dropped, its inner value is cleaned up with
/// [`UA_Client_delete()`].
pub struct Client(NonNull<UA_Client>);

// SAFETY: We know that the underlying `UA_Client` allows access from different threads (at least as
// long as we do not call functions concurrently).
unsafe impl Send for Client {}

impl Client {
    /// Returns const pointer to value.
    ///
    /// # Safety
    ///
    /// The value is owned by `Self`. Ownership must not be given away, in whole or in parts. This
    /// may happen when `open62541` functions are called that take ownership of values by pointer.
    #[allow(dead_code)]
    #[must_use]
    pub(crate) const unsafe fn as_ptr(&self) -> *const UA_Client {
        self.0.as_ptr()
    }

    /// Returns mutable pointer to value.
    ///
    /// # Safety
    ///
    /// The value is owned by `Self`. Ownership must not be given away, in whole or in parts. This
    /// may happen when `open62541` functions are called that take ownership of values by pointer.
    #[must_use]
    pub(crate) unsafe fn as_mut_ptr(&mut self) -> *mut UA_Client {
        self.0.as_ptr()
    }

    /// Gets current channel and session state, and connect status.
    #[allow(dead_code)] // --no-default-features
    pub(crate) fn state(&mut self) -> ClientState {
        log::debug!("Getting state");

        let mut channel_state = ua::SecureChannelState::init();
        let mut session_state = ua::SessionState::init();
        let mut connect_status = ua::StatusCode::init();

        unsafe {
            UA_Client_getState(
                self.as_mut_ptr(),
                channel_state.as_mut_ptr(),
                session_state.as_mut_ptr(),
                connect_status.as_mut_ptr(),
            );
        }

        ClientState {
            channel_state,
            session_state,
            connect_status,
        }
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        log::info!("Disconnecting from endpoint");

        // Disconnection is always performed synchronously (with blocking), for the server to handle
        // the CloseSession request. Looking at the implementation of `UA_Client_disconnect()`, this
        // seems to be done with a timeout of 10 seconds.
        //
        // TODO: Refactor this to avoid blocking in `drop()` for a potentially long time.
        let status_code = ua::StatusCode::new(unsafe { UA_Client_disconnect(self.as_mut_ptr()) });
        if let Err(error) = Error::verify_good(&status_code) {
            log::warn!("Error while disconnecting client: {error}");
        }

        log::debug!("Deleting client");

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
