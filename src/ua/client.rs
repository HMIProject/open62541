use std::ptr::NonNull;

use open62541_sys::{
    UA_Client, UA_Client_delete, UA_Client_disconnect, UA_Client_getConfig, UA_Client_getContext,
    UA_Client_getState, UA_Client_new, UA_Client_newWithConfig,
};

use crate::{ClientContext, DataType as _, Error, ua};

/// Combined state for [`Client`] and [`AsyncClient`].
///
/// [`AsyncClient`]: crate::AsyncClient
#[derive(Debug)]
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
#[derive(Debug)]
pub struct Client(NonNull<UA_Client>);

// SAFETY: We know that the underlying `UA_Client` allows access from different threads, i.e. it may
// be dropped in a different thread from where it was created.
unsafe impl Send for Client {}

// SAFETY: The underlying `UA_Client` can be used from different threads concurrently, at least with
// _most_ methods (those marked `UA_THREADSAFE` and/or with explicit mutex locks inside).
unsafe impl Sync for Client {}

impl Client {
    /// Creates client from client config.
    ///
    /// This consumes the config object and makes the client the owner of all contained data therein
    /// (e.g. logging configuration and logger instance).
    pub(crate) fn new_with_config(config: ua::ClientConfig) -> Self {
        let config = config.into_raw();
        let inner = unsafe { UA_Client_newWithConfig(&raw const config) };
        // PANIC: The only possible errors here are out-of-memory.
        let inner = NonNull::new(inner).expect("create UA_Client");
        Self(inner)
    }

    /// Returns const pointer to value.
    ///
    /// # Safety
    ///
    /// The value is owned by `Self`. Ownership must not be given away, in whole or in parts. This
    /// may happen when `open62541` functions are called that take ownership of values by pointer.
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
    #[expect(clippy::allow_attributes, reason = "non-static condition")]
    #[allow(clippy::missing_const_for_fn, reason = "unsupported before Rust 1.87")]
    pub(crate) unsafe fn as_mut_ptr(&mut self) -> *mut UA_Client {
        self.0.as_ptr()
    }

    /// Gets current channel and session state, and connect status.
    #[must_use]
    pub(crate) fn state(&self) -> ClientState {
        log::debug!("Getting state");

        let mut channel_state = ua::SecureChannelState::init();
        let mut session_state = ua::SessionState::init();
        let mut connect_status = ua::StatusCode::init();

        unsafe {
            UA_Client_getState(
                // SAFETY: Cast to `mut` pointer, function is marked `UA_THREADSAFE`.
                self.as_ptr().cast_mut(),
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

    /// Disconnects from endpoint.
    pub(crate) fn disconnect(mut self) {
        log::info!("Disconnecting from endpoint");

        let status_code = ua::StatusCode::new(unsafe {
            // SAFETY: We retain ownership of `self`.
            UA_Client_disconnect(self.as_mut_ptr())
        });
        if let Err(error) = Error::verify_good(&status_code) {
            log::warn!("Error while disconnecting client: {error}");
        }
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        log::debug!("Deleting client");

        // Fetch context pointer before deleting client below, but free associated memory only after
        // client has completely shut down.
        let context = unsafe { UA_Client_getContext(self.as_mut_ptr()) }.cast::<ClientContext>();

        // `UA_Client_delete()` matches `UA_Client_new()`. This may block (!) whenever the client is
        // still connected, for as long as it takes to take down the connection. This can be avoided
        // by calling `disconnect()` instead of simply dropping the client.
        unsafe { UA_Client_delete(self.as_mut_ptr()) }

        // Reclaim wrapped client context to avoid leaking memory. This simply drops the value. Note
        // that the context may be null if (and only if) the client was default-initialized (instead
        // of going through `ua::ClientConfig`).
        if !context.is_null() {
            let _context: Box<ClientContext> = unsafe { Box::from_raw(context) };
        }
    }
}

impl Default for Client {
    fn default() -> Self {
        // `UA_Client_new()` matches `UA_Client_delete()`.
        let inner = NonNull::new(unsafe { UA_Client_new() }).expect("create UA_Client");

        // For default-initialized clients, context must be unset.
        debug_assert!({
            let config = unsafe { UA_Client_getConfig(inner.as_ptr()) };
            let config = unsafe { config.as_mut() }.expect("client config must be set");
            config.clientContext.is_null()
        });

        Self(inner)
    }
}
