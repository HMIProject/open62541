use std::{
    ffi::{c_char, c_ulong, c_void, CStr, CString},
    ptr,
};

use open62541_sys::{
    vsnprintf, UA_ClientConfig, UA_ClientConfig_setDefault, UA_Client_connect, UA_Client_getConfig,
    UA_LogCategory, UA_LogLevel, UA_STATUSCODE_GOOD,
};

use crate::{ua, Error};

/// Builder for [`Client`].
///
/// Use this to specify additional options before connecting to an OPC UA endpoint.
#[allow(clippy::module_name_repetitions)]
pub struct ClientBuilder(ua::Client);

impl ClientBuilder {
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

        // We require some initial configuration `UA_Client_connect()` to work.
        //
        let result = unsafe {
            let config = UA_Client_getConfig(inner.as_mut_ptr());

            // Install custom logger that uses `log` crate.
            set_default_logger(config.as_mut().expect("client config should be set"));

            // Setting the remainder of the configuration to defaults keeps our custom logger. Do so
            // after setting the logger to prevent this call to install another default logger which
            // we would throw away in `set_default_logger()` anyway.
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
    /// `cycle_time` controls the frequency at which the client will poll the server
    /// for responses in the background.
    ///
    /// [`AsyncClient`]: crate::AsyncClient
    #[cfg(feature = "tokio")]
    #[must_use]
    pub fn into_async(self, cycle_time: tokio::time::Duration) -> crate::AsyncClient {
        crate::AsyncClient::from_sync(self.0, cycle_time)
    }
}

/// Installs logger that forwards to `log` crate.
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
        // For some reason, the magic is necessary to accommodate the different signatures generated
        // by `bindgen` in `open62541-sys`.
        #[cfg(all(unix, target_arch = "x86_64"))] args: *mut open62541_sys::__va_list_tag,
        #[cfg(not(all(unix, target_arch = "x86_64")))] args: open62541_sys::va_list,
    ) {
        // We delegate string formatting to `vsnprintf()`, the length-checked string buffer variant
        // of the variadic `vprintf` family. We call the function twice: first to get the length of
        // the resulting string, then to actually fill in the string with data.

        // Result is the number of bytes that would be written not including the NUL terminator.
        let msg_length = usize::try_from(unsafe { vsnprintf(ptr::null_mut(), 0, msg, args) })
            .expect("string length should fit into memory");

        // Allocate buffer with one more byte for NUL terminator.
        let mut msg_buffer: Vec<u8> = vec![0; msg_length + 1];

        // Fill in string with data.
        unsafe {
            vsnprintf(
                msg_buffer.as_mut_ptr().cast::<i8>(),
                c_ulong::try_from(msg_buffer.len()).expect("string length should fit into memory"),
                msg,
                args,
            )
        };

        // This includes a final check that the string has the expected length (i.e. the buffer was
        // filled completely, ending in a NUL terminator).
        let msg = CStr::from_bytes_with_nul(&msg_buffer)
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
            // TODO: Handle unexpected level.
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
