use std::ptr::NonNull;

use open62541_sys::{UA_Client, UA_Client_delete, UA_Client_getState, UA_Client_new};

use crate::{ua, DataType as _};

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
    #[allow(dead_code)]
    #[must_use]
    pub(crate) const fn as_ptr(&self) -> *const UA_Client {
        self.0.as_ptr()
    }

    #[must_use]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut UA_Client {
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
