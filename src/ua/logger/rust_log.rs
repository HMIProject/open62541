use std::{
    ffi::{c_char, c_void, CStr},
    ptr,
};

use open62541_sys::{vsnprintf_va_copy, vsnprintf_va_end, UA_LogCategory, UA_LogLevel, UA_Logger};

use crate::ua;

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
        let Some(msg) = format_message(msg, args) else {
            log::error!(target: LOG_TARGET, "Unknown log message");
            return;
        };

        let msg = CStr::from_bytes_with_nul(&msg)
            .unwrap_or(c"Invalid log message")
            .to_string_lossy();

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
    let logger = Box::leak(Box::new(UA_Logger {
        log: Some(log_c),
        context: ptr::null_mut(),
        clear: Some(clear_c),
    }));

    // SAFETY: We created the logger instance.
    unsafe { ua::Logger::from_raw(logger) }
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
