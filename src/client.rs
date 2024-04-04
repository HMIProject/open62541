use std::{
    ffi::{c_char, c_void, CStr, CString},
    ptr,
    time::Duration,
};

use open62541_sys::{
    vsnprintf_va_copy, vsnprintf_va_end, UA_ClientConfig, UA_Client_connect, UA_LogCategory,
    UA_LogLevel, UA_Logger,
};

use crate::{ua, DataType as _, Error, Result};

/// Builder for [`Client`].
///
/// Use this to specify additional options before connecting to an OPC UA endpoint.
///
/// # Examples
///
/// ```no_run
/// use open62541::ClientBuilder;
/// use std::time::Duration;
///
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() -> anyhow::Result<()> {
/// #
/// let client = ClientBuilder::default()
///     .secure_channel_life_time(Duration::from_secs(60))
///     .connect("opc.tcp://opcuademo.sterfive.com:26543")?;
/// #
/// # Ok(())
/// # }
/// ```
#[derive(Default)]
#[allow(clippy::module_name_repetitions)]
pub struct ClientBuilder(ua::ClientConfig);

impl ClientBuilder {
    /// Sets (response) timeout.
    ///
    /// # Panics
    ///
    /// The given duration must be non-negative and less than 4,294,967,295 milliseconds (less than
    /// 49.7 days).
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config_mut().timeout = u32::try_from(timeout.as_millis())
            .expect("timeout (in milliseconds) should be in range of u32");
        self
    }

    /// Sets client description.
    ///
    /// The description must be internally consistent. The application URI set in the application
    /// description must match the URI set in the certificate.
    #[must_use]
    pub fn client_description(mut self, client_description: ua::ApplicationDescription) -> Self {
        client_description.move_into_raw(&mut self.config_mut().clientDescription);
        self
    }

    /// Sets secure channel life time.
    ///
    /// After this life time, the channel needs to be renewed.
    ///
    /// # Panics
    ///
    /// The given duration must be non-negative and less than 4,294,967,295 milliseconds (less than
    /// 49.7 days).
    #[must_use]
    pub fn secure_channel_life_time(mut self, secure_channel_life_time: Duration) -> Self {
        self.config_mut().secureChannelLifeTime =
            u32::try_from(secure_channel_life_time.as_millis())
                .expect("secure channel life time (in milliseconds) should be in range of u32");
        self
    }

    /// Sets requested session timeout.
    ///
    /// # Panics
    ///
    /// The given duration must be non-negative and less than 4,294,967,295 milliseconds (less than
    /// 49.7 days).
    #[must_use]
    pub fn requested_session_timeout(mut self, requested_session_timeout: Duration) -> Self {
        self.config_mut().requestedSessionTimeout =
            u32::try_from(requested_session_timeout.as_millis())
                .expect("secure channel life time (in milliseconds) should be in range of u32");
        self
    }

    /// Sets connectivity check interval.
    ///
    /// Use `None` to disable background task.
    ///
    /// # Panics
    ///
    /// The given duration must be non-negative and less than 4,294,967,295 milliseconds (less than
    /// 49.7 days).
    #[must_use]
    pub fn connectivity_check_interval(
        mut self,
        connectivity_check_interval: Option<Duration>,
    ) -> Self {
        self.config_mut().connectivityCheckInterval =
            u32::try_from(connectivity_check_interval.map_or(0, |interval| interval.as_millis()))
                .expect("connectivity check interval (in milliseconds) should be in range of u32");
        self
    }

    /// Connects to OPC UA endpoint and returns [`Client`].
    ///
    /// # Errors
    ///
    /// This fails when the target server is not reachable.
    ///
    /// # Panics
    ///
    /// The endpoint URL must not contain any NUL bytes.
    pub fn connect(self, endpoint_url: &str) -> Result<Client> {
        log::info!("Connecting to endpoint {endpoint_url}");

        let endpoint_url =
            CString::new(endpoint_url).expect("endpoint URL does not contain NUL bytes");

        let mut client = ua::Client::new_with_config(self.0);

        let status_code = ua::StatusCode::new(unsafe {
            UA_Client_connect(client.as_mut_ptr(), endpoint_url.as_ptr())
        });
        Error::verify_good(&status_code)?;

        Ok(Client(client))
    }

    /// Access client configuration.
    fn config_mut(&mut self) -> &mut UA_ClientConfig {
        // SAFETY: Ownership is not given away.
        unsafe { self.0.as_mut() }
    }
}

/// Connected OPC UA client.
///
/// This represents an OPC UA client connected to a specific endpoint. Once a client is connected to
/// an endpoint, it is not possible to switch to another server. Create a new client for that.
///
/// Once a connection to the given endpoint is established, the client keeps the connection open and
/// reconnects if necessary.
///
/// If the connection fails unrecoverably, the client is no longer usable. In this case create a new
/// client if required.
///
/// To disconnect, prefer method [`disconnect()`](Self::disconnect) over simply dropping the client:
/// disconnection involves server communication and might take a short amount of time.
pub struct Client(
    #[allow(dead_code)] // --no-default-features
    ua::Client,
);

impl Client {
    /// Creates client connected to endpoint.
    ///
    /// If you need more control over the initialization, use [`ClientBuilder`] instead, and turn it
    /// into [`Client`] by calling [`connect()`](ClientBuilder::connect).
    ///
    /// # Errors
    ///
    /// See [`ClientBuilder::connect()`].
    ///
    /// # Panics
    ///
    /// See [`ClientBuilder::connect()`].
    pub fn new(endpoint_url: &str) -> Result<Self> {
        ClientBuilder::default().connect(endpoint_url)
    }

    /// Turns client into [`AsyncClient`].
    ///
    /// The [`AsyncClient`] can be used to access methods in an asynchronous way.
    ///
    /// `cycle_time` controls the frequency at which the client will poll the server for responses
    /// in the background.
    ///
    /// [`AsyncClient`]: crate::AsyncClient
    #[cfg(feature = "tokio")]
    #[must_use]
    pub fn into_async(self, cycle_time: tokio::time::Duration) -> crate::AsyncClient {
        crate::AsyncClient::from_sync(self.0, cycle_time)
    }

    /// Gets current channel and session state, and connect status.
    #[must_use]
    pub fn state(&self) -> ua::ClientState {
        self.0.state()
    }

    /// Disconnects from endpoint.
    ///
    /// This consumes the client and handles the graceful shutdown of the connection. This should be
    /// preferred over simply dropping the instance to give the server a chance to clean up and also
    /// to avoid blocking unexpectedly when the client is being dropped without calling this method.
    // Forward any result as-is to detect mismatching method signatures at compile time if the
    // return type of the inner method should ever change.
    #[allow(clippy::semicolon_if_nothing_returned)]
    pub fn disconnect(self) {
        self.0.disconnect()
    }
}

/// Creates logger that forwards to the `log` crate.
///
/// We can use this to prevent `open62541` from installing its own default logger (which outputs any
/// logs to stdout/stderr directly).
///
/// Note that this leaks memory unless the returned pointer is assigned to `UA_ClientConfig` (and/or
/// `UA_Client` in turn), eventually calling `UA_Logger::clear()` with this `UA_Logger` instance.
pub(crate) fn client_logger() -> *mut UA_Logger {
    unsafe extern "C" fn log_c(
        _log_context: *mut c_void,
        level: UA_LogLevel,
        _category: UA_LogCategory,
        msg: *const c_char,
        args: open62541_sys::va_list_,
    ) {
        let Some(msg) = format_message(msg, args) else {
            log::error!("Unknown log message");
            return;
        };

        let msg = CStr::from_bytes_with_nul(&msg)
            .expect("string length should match")
            .to_string_lossy();

        if level == UA_LogLevel::UA_LOGLEVEL_FATAL {
            // Without fatal level in `log`, fall back to error.
            log::error!("{msg}");
        } else if level == UA_LogLevel::UA_LOGLEVEL_ERROR {
            log::error!("{msg}");
        } else if level == UA_LogLevel::UA_LOGLEVEL_WARNING {
            log::warn!("{msg}");
        } else if level == UA_LogLevel::UA_LOGLEVEL_INFO {
            log::info!("{msg}");
        } else if level == UA_LogLevel::UA_LOGLEVEL_DEBUG {
            log::debug!("{msg}");
        } else if level == UA_LogLevel::UA_LOGLEVEL_TRACE {
            log::trace!("{msg}");
        } else {
            // Handle unexpected level by escalating to error.
            log::error!("{msg}");
        }
    }

    unsafe extern "C" fn clear_c(logger: *mut UA_Logger) {
        log::debug!("Clearing `log` logger");

        // This consumes the `UA_Logger` structure itself, invalidating the pointer `config.logging`
        // and thereby releasing all allocated resources.
        //
        // This is in line with the contract that `config.logging` may not be used anymore after its
        // `clear()` method has been called.
        let logger = unsafe { Box::from_raw(logger) };

        // Run some sanity checks. We should only ever be called on our own data structure.
        debug_assert!(logger.log == Some(log_c));
        debug_assert!(logger.clear == Some(clear_c));

        // As long as we do not carry data, there is nothing to clean up here.
        debug_assert!(logger.context.is_null());

        // Dropping the boxed logger cleans up allocated memory.
        drop(logger);
    }

    log::debug!("Creating `log` logger");

    // Create logger configuration. We leak the memory which is cleaned up eventually when `clear()`
    // is called (which is `clear_c()` above).
    Box::leak(Box::new(UA_Logger {
        log: Some(log_c),
        context: ptr::null_mut(),
        clear: Some(clear_c),
    }))
}

/// Initial buffer size when formatting messages.
const FORMAT_MESSAGE_DEFAULT_BUFFER_LEN: usize = 128;

/// Maximum buffer size when formatting messages.
const FORMAT_MESSAGE_MAXIMUM_BUFFER_LEN: usize = 65536;

/// Formats message with `vprintf` library calls.
///
/// This returns the formatted message with a trailing NUL byte, or `None` when formatting fails. A
/// long message may be truncated (see [`FORMAT_MESSAGE_MAXIMUM_BUFFER_LEN`] for details); its last
/// characters will be replaced with `...` to indicate this.
fn format_message(msg: *const c_char, args: open62541_sys::va_list_) -> Option<Vec<u8>> {
    // Delegate string formatting to `vsnprintf()`, the length-checked string buffer variant of the
    // variadic `vprintf` family.
    //
    // We use the custom `vsnprintf_va_copy()` provided by `open62541_sys`. This copies the va args
    // and requires an explicit call to `vsnprintf_va_end()` afterwards.

    // Allocate default buffer first. Only when the message doesn't fit, we need to allocate larger
    // buffer below.
    let mut msg_buffer: Vec<u8> = vec![0; FORMAT_MESSAGE_DEFAULT_BUFFER_LEN];
    loop {
        let result = unsafe {
            vsnprintf_va_copy(
                msg_buffer.as_mut_ptr().cast::<c_char>(),
                msg_buffer.len(),
                msg,
                args,
            )
        };
        let Ok(msg_len) = usize::try_from(result) else {
            // Negative result is an error in the format string. Nothing we can do.
            debug_assert!(result < 0);
            // Free the `va_list` argument that is no consumed by `vsnprintf()`!
            unsafe { vsnprintf_va_end(args) }
            return None;
        };
        let buffer_len = msg_len + 1;
        if buffer_len > msg_buffer.len() {
            // Last byte must always be the NUL terminator, even if the message
            // doesn't fit into the buffer.
            debug_assert_eq!(msg_buffer.last(), Some(&0));
            if msg_buffer.len() < FORMAT_MESSAGE_MAXIMUM_BUFFER_LEN {
                // Allocate larger buffer and try again.
                msg_buffer.resize(FORMAT_MESSAGE_MAXIMUM_BUFFER_LEN, 0);
                continue;
            }
            // Message is too large to format. Truncate the message by ending it with `...`.
            for char in msg_buffer.iter_mut().rev().skip(1).take(3) {
                *char = b'.';
            }
        } else {
            // Message fits into the buffer. Make sure that `from_bytes_with_nul()`
            // sees the expected single NUL terminator in the final position.
            msg_buffer.truncate(buffer_len);
        }
        break;
    }

    // Free the `va_list` argument that is not consumed by `vsnprintf()`!
    unsafe { vsnprintf_va_end(args) }

    // Last byte must always be the NUL terminator.
    debug_assert_eq!(msg_buffer.last(), Some(&0));

    Some(msg_buffer)
}
