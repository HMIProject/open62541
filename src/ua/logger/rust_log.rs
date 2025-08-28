use std::{
    ffi::{c_char, c_void},
    ptr,
};

use open62541_sys::{UA_LogCategory, UA_LogLevel, UA_Logger, UA_String_vformat};

use crate::{ua, DataType as _, Error, Result};

const LOG_TARGET: &str = "open62541_sys";

/// Creates logger that forwards to the `log` crate.
///
/// We can use this to prevent `open62541` from installing its own default logger (which outputs any
/// logs to stdout/stderr directly).
pub(crate) fn logger() -> ua::Logger {
    unsafe extern "C" fn log_c(
        _log_context: *mut c_void,
        level: UA_LogLevel,
        _category: UA_LogCategory,
        msg: *const c_char,
        args: open62541_sys::va_list_,
    ) {
        let msg = match format_message(msg, args) {
            Ok(msg) => msg,
            Err(error) => {
                log::error!(target: LOG_TARGET, "Unknown log message: {error}");
                return;
            }
        };

        let msg = msg.as_str().unwrap_or("Invalid log message");

        if level == UA_LogLevel::UA_LOGLEVEL_FATAL {
            // Without fatal level in `log`, fall back to error.
            log::error!(target: LOG_TARGET, "{msg}");
        } else if level == UA_LogLevel::UA_LOGLEVEL_ERROR {
            log::error!(target: LOG_TARGET, "{msg}");
        } else if level == UA_LogLevel::UA_LOGLEVEL_WARNING {
            log::warn!(target: LOG_TARGET, "{msg}");
        } else if level == UA_LogLevel::UA_LOGLEVEL_INFO {
            log::info!(target: LOG_TARGET, "{msg}");
        } else if level == UA_LogLevel::UA_LOGLEVEL_DEBUG {
            log::debug!(target: LOG_TARGET, "{msg}");
        } else if level == UA_LogLevel::UA_LOGLEVEL_TRACE {
            log::trace!(target: LOG_TARGET, "{msg}");
        } else {
            // Handle unexpected level by escalating to error.
            log::error!(target: LOG_TARGET, "{msg}");
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
        //
        // TODO: Use `std::ptr::fn_addr_eq()` when MSRV has been upgraded to Rust 1.85.
        #[expect(unpredictable_function_pointer_comparisons, reason = "MSRV 1.83")]
        {
            debug_assert!(logger.log == Some(log_c));
            debug_assert!(logger.clear == Some(clear_c));
        }

        // As long as we do not carry data, there is nothing to clean up here.
        debug_assert!(logger.context.is_null());

        // Dropping the boxed logger cleans up allocated memory.
        drop(logger);
    }

    log::debug!("Creating `log` logger");

    // Create logger configuration. We leak the memory which is cleaned up eventually when `clear()`
    // is called (which is `clear_c()` above).
    let logger = Box::leak(Box::new(UA_Logger {
        log: Some(log_c),
        context: ptr::null_mut(),
        clear: Some(clear_c),
    }));

    // SAFETY: We created the logger instance.
    unsafe { ua::Logger::from_raw(logger) }
}

/// Buffer size when formatting messages.
// This matches the limit used by default implementations `ua_log_stdout.h` and `ua_log_syslog.h`.
const FORMAT_MESSAGE_BUFFER_LEN: usize = 512;

/// Formats message with `vprintf` library calls.
///
/// This returns the formatted message as string, or `Err` when formatting fails. A long message is
/// truncated (see [`FORMAT_MESSAGE_BUFFER_LEN`]); its last characters will be replaced with `...`.
fn format_message(msg: *const c_char, args: open62541_sys::va_list_) -> Result<ua::String> {
    // With non-zero length, `UA_String_vformat()` fills the given string directly. For zero length
    // strings, the result would be dynamically allocated, but this risks handling incredibly large
    // amounts of memory in the log handler which we want to avoid here.
    let mut msg_buffer = ua::String::uninit(FORMAT_MESSAGE_BUFFER_LEN);

    let status_code =
        ua::StatusCode::new(unsafe { UA_String_vformat(msg_buffer.as_mut_ptr(), msg, args) });
    if status_code == ua::StatusCode::BADENCODINGLIMITSEXCEEDED {
        // Message is too large to format. We could try again with a larger buffer, but since we do
        // not know the required length (`UA_String_vformat()` doesn't return it), we would have to
        // guess (e.g., doubling the length until the message fits). Simply truncate the message by
        // ending it with `...` instead to ensure constant-time operation.
        if let Some(msg_buffer) = msg_buffer.as_mut_bytes() {
            for char in msg_buffer.iter_mut().rev().take(3) {
                *char = b'.';
            }
        }
    } else {
        Error::verify_good(&status_code)?;
    }

    Ok(msg_buffer)
}
