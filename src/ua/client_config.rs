use std::{
    ffi::{c_void, CStr},
    mem::MaybeUninit,
    os::raw::c_char,
    ptr,
};

use open62541_sys::{
    vsnprintf_va_copy, vsnprintf_va_end, UA_ClientConfig, UA_ClientConfig_clear,
    UA_ClientConfig_setDefault, UA_LogCategory, UA_LogLevel, UA_Logger,
};

use crate::{ua, Error};

pub(crate) struct ClientConfig(Option<UA_ClientConfig>);

impl ClientConfig {
    /// Creates wrapper by taking ownership of value.
    ///
    /// When `Self` is dropped, allocations held by the inner type are cleaned up.
    ///
    /// # Safety
    ///
    /// Ownership of the value passes to `Self`. This must only be used for values that are not
    /// contained within other values that may be dropped.
    #[must_use]
    pub(crate) const unsafe fn from_raw(src: UA_ClientConfig) -> Self {
        Self(Some(src))
    }

    /// Gives up ownership and returns value.
    ///
    /// The returned value must be re-wrapped with [`from_raw()`], cleared manually, or copied into
    /// an owning value (like [`UA_Client`]) to free internal allocations and not leak memory.
    ///
    /// [`from_raw()`]: Self::from_raw
    /// [`UA_Client`]: open62541_sys::UA_Client
    #[must_use]
    pub(crate) fn into_raw(mut self) -> UA_ClientConfig {
        self.0.take().expect("should have client config")
    }

    /// Creates wrapper initialized with defaults.
    ///
    /// This initializes the value and makes all attributes well-defined. Additional attributes may
    /// need to be initialized for the value to be actually useful afterwards.
    pub(crate) const fn init() -> Self {
        let inner = MaybeUninit::<UA_ClientConfig>::zeroed();
        // SAFETY: Zero-initialized memory is a valid client config.
        let inner = unsafe { inner.assume_init() };
        // SAFETY: We pass a value without pointers to it into `Self`.
        unsafe { Self::from_raw(inner) }
    }

    /// Returns exclusive reference to value.
    ///
    /// # Safety
    ///
    /// The value is owned by `Self`. Ownership must not be given away, in whole or in parts. This
    /// may happen when `open62541` functions are called that take ownership of values by pointer.
    #[must_use]
    pub(crate) unsafe fn as_mut(&mut self) -> &mut UA_ClientConfig {
        // PANIC: The inner object can only be unset when ownership has been given away.
        self.0.as_mut().expect("should have client config")
    }

    /// Returns mutable pointer to value.
    ///
    /// # Safety
    ///
    /// The value is owned by `Self`. Ownership must not be given away, in whole or in parts. This
    /// may happen when `open62541` functions are called that take ownership of values by pointer.
    #[must_use]
    pub(crate) unsafe fn as_mut_ptr(&mut self) -> *mut UA_ClientConfig {
        // PANIC: The inner object can only be unset when ownership has been given away.
        self.0.as_mut().expect("should have client config")
    }
}

impl Drop for ClientConfig {
    fn drop(&mut self) {
        // Check if we still hold the client config. If not, we must not clean up: the ownership has
        // passed to the client that was created from this config.
        let Some(mut inner) = self.0.take() else {
            return;
        };

        // Clean up held resources such as logging configuration.
        unsafe { UA_ClientConfig_clear(&mut inner) }
    }
}

impl Default for ClientConfig {
    fn default() -> Self {
        let mut config = Self::init();

        // First set custom logger. This is necessary because the same logger instance is used as-is
        // inside derived attributes such as `eventLoop`, `certificateVerification`, etc.
        set_default_logger(unsafe { config.as_mut() });

        // Set remaining attributes to their default values. This also copies the logger as laid out
        // above.
        let status_code =
            ua::StatusCode::new(unsafe { UA_ClientConfig_setDefault(config.as_mut_ptr()) });
        // PANIC: The only possible errors here are out-of-memory.
        Error::verify_good(&status_code).expect("should be able to set default client config");

        config
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
            log::error!(target: "open62541::client", "Unknown log message");
            return;
        };

        let msg = CStr::from_bytes_with_nul(&msg)
            .expect("string length should match")
            .to_string_lossy();

        if level == UA_LogLevel::UA_LOGLEVEL_FATAL {
            // Without fatal level in `log`, fall back to error.
            log::error!(target: "open62541::client", "{msg}");
        } else if level == UA_LogLevel::UA_LOGLEVEL_ERROR {
            log::error!(target: "open62541::client", "{msg}");
        } else if level == UA_LogLevel::UA_LOGLEVEL_WARNING {
            log::warn!(target: "open62541::client", "{msg}");
        } else if level == UA_LogLevel::UA_LOGLEVEL_INFO {
            log::info!(target: "open62541::client", "{msg}");
        } else if level == UA_LogLevel::UA_LOGLEVEL_DEBUG {
            log::debug!(target: "open62541::client", "{msg}");
        } else if level == UA_LogLevel::UA_LOGLEVEL_TRACE {
            log::trace!(target: "open62541::client", "{msg}");
        } else {
            // Handle unexpected level by escalating to error.
            log::error!(target: "open62541::client", "{msg}");
        }
    }

    unsafe extern "C" fn clear_c(logger: *mut UA_Logger) {
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

    // This function should only be called on default-initialized config objects. The reason is that
    // we do not know whether the `logging` configuration is still referenced somewhere else.
    assert!(config.logging.is_null());

    // Create logger configuration. We leak the memory which is cleaned up eventually when `clear()`
    // is called (which is `clear_c()` above).
    config.logging = Box::leak(Box::new(UA_Logger {
        log: Some(log_c),
        context: ptr::null_mut(),
        clear: Some(clear_c),
    }));
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
