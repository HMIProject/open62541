use std::{
    ffi::{c_char, c_void, CStr, CString},
    ptr,
    time::Duration,
};

use open62541_sys::{
    vsnprintf_va_copy, vsnprintf_va_end, UA_ClientConfig, UA_ClientConfig_setDefault,
    UA_Client_connect, UA_Client_getConfig, UA_LogCategory, UA_LogLevel, UA_STATUSCODE_GOOD,
};

use crate::{ua, Error};

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
///     .secure_channel_lifetime(Duration::from_secs(60))
///     .connect("opc.tcp://opcuademo.sterfive.com:26543")?;
/// #
/// # Ok(())
/// # }
/// ```
#[allow(clippy::module_name_repetitions)]
pub struct ClientBuilder(ua::Client);

impl ClientBuilder {
    /// Sets secure channel life time.
    ///
    /// # Panics
    ///
    /// The given duration must be non-negative and less than 4,294,967,295 milliseconds (less than
    /// 49.7 days).
    #[must_use]
    pub fn secure_channel_lifetime(mut self, secure_channel_lifetime: Duration) -> Self {
        let config = unsafe { UA_Client_getConfig(self.0.as_mut_ptr()).as_mut() };

        config.unwrap().secureChannelLifeTime = u32::try_from(secure_channel_lifetime.as_millis())
            .expect("secure channel life time should be in range of u32");

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
        let config = unsafe { UA_Client_getConfig(self.0.as_mut_ptr()).as_mut() };

        config.unwrap().requestedSessionTimeout =
            u32::try_from(requested_session_timeout.as_millis())
                .expect("secure channel life time should be in range of u32");

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
    pub fn connect(mut self, endpoint_url: &str) -> Result<Client, Error> {
        log::info!("Connecting to endpoint {endpoint_url}");

        let endpoint_url =
            CString::new(endpoint_url).expect("endpoint URL does not contain NUL bytes");

        let status_code = ua::StatusCode::new(unsafe {
            UA_Client_connect(self.0.as_mut_ptr(), endpoint_url.as_ptr())
        });
        Error::verify_good(&status_code)?;

        Ok(Client(self.0))
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        let mut inner = ua::Client::default();

        // We require some initial configuration for `UA_Client_connect()` to work.
        //
        let result = unsafe {
            let config = UA_Client_getConfig(inner.as_mut_ptr());

            // Install custom logger that uses the `log` crate.
            set_default_logger(config.as_mut().expect("client config should be set"));

            // Initialize remainder of configuration with defaults. This keeps our custom logger. We
            // do this after `set_default_logger()`: `UA_ClientConfig_setDefault()` would needlessly
            // install a default logger that we would throw away in `set_default_logger()` anyway.
            UA_ClientConfig_setDefault(config)
        };
        assert!(result == UA_STATUSCODE_GOOD);

        Self(inner)
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
pub struct Client(ua::Client);

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
    pub fn new(endpoint_url: &str) -> Result<Self, Error> {
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
}

/// Installs logger that forwards to the `log` crate.
///
/// This remove an existing logger from the given configuration (by calling its `clear()` callback),
/// then installs a custom logger that forwards all messages to the corresponding calls in the `log`
/// crate.
///
/// We can use this to prevent `open62541` from installing its own default logger (which outputs any
/// logs to stdout/stderr directly).
fn set_default_logger(config: &mut UA_ClientConfig) {
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

    // Reset existing logger configuration.
    if let Some(clear) = config.logger.clear {
        unsafe { clear(config.logger.context) };
    }

    // Set logger configuration to our own.
    config.logger.clear = None;
    config.logger.log = Some(log_c);
    config.logger.context = ptr::null_mut();
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
