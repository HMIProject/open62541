use std::{
    borrow::Cow,
    ffi::{c_char, c_void, CStr},
    ptr,
};

use log::Level;
use open62541_sys::{vsnprintf_va_copy, vsnprintf_va_end, UA_LogCategory, UA_LogLevel, UA_Logger};

use crate::ua;

// This matches the crate name.
const LOG_TARGET: &str = "open62541_sys";

/// Creates logger that forwards to the `log` crate.
///
/// We can use this to prevent `open62541` from installing its own default logger (which outputs all
/// logs to stdout/stderr directly).
pub(crate) fn logger() -> ua::Logger {
    unsafe extern "C" fn log_c(
        _log_context: *mut c_void,
        level: UA_LogLevel,
        category: UA_LogCategory,
        msg: *const c_char,
        args: open62541_sys::va_list_,
    ) {
        let level = match level {
            // Without fatal level in `log`, fall back to error.
            UA_LogLevel::UA_LOGLEVEL_FATAL | UA_LogLevel::UA_LOGLEVEL_ERROR => Level::Error,
            UA_LogLevel::UA_LOGLEVEL_WARNING => Level::Warn,
            UA_LogLevel::UA_LOGLEVEL_INFO => Level::Info,
            UA_LogLevel::UA_LOGLEVEL_DEBUG => Level::Debug,
            UA_LogLevel::UA_LOGLEVEL_TRACE => Level::Trace,
            // Handle unexpected level by escalating to error.
            #[expect(clippy::match_same_arms, reason = "distinction of cases")]
            _ => Level::Error,
        };

        if !log::log_enabled!(target: LOG_TARGET, level) {
            // Bail out early to skip formatting message.
            return;
        }

        let msg = format_message(msg, args);
        let msg = match msg {
            Some(ref msg) => CStr::from_bytes_with_nul(msg)
                .unwrap_or(c"Invalid log message")
                .to_string_lossy(),
            None => Cow::Borrowed("Unknown log message"),
        };

        let category = log_category(&category);

        log::log!(target: LOG_TARGET, level, "({category}) {msg}");
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

// These match the category names from `ua_log_stdout.c` and `ua_log_syslog.c`.
const LOG_CATEGORY_NETWORK: &str = "network";
const LOG_CATEGORY_SECURECHANNEL: &str = "channel";
const LOG_CATEGORY_SESSION: &str = "session";
const LOG_CATEGORY_SERVER: &str = "server";
const LOG_CATEGORY_CLIENT: &str = "client";
const LOG_CATEGORY_USERLAND: &str = "userland";
const LOG_CATEGORY_SECURITYPOLICY: &str = "security";
const LOG_CATEGORY_EVENTLOOP: &str = "eventloop";
const LOG_CATEGORY_PUBSUB: &str = "pubsub";
const LOG_CATEGORY_DISCOVERY: &str = "discovery";
const LOG_CATEGORY_UNKNOWN: &str = "unknown";

const fn log_category(category: &UA_LogCategory) -> &'static str {
    match *category {
        UA_LogCategory::UA_LOGCATEGORY_NETWORK => LOG_CATEGORY_NETWORK,
        UA_LogCategory::UA_LOGCATEGORY_SECURECHANNEL => LOG_CATEGORY_SECURECHANNEL,
        UA_LogCategory::UA_LOGCATEGORY_SESSION => LOG_CATEGORY_SESSION,
        UA_LogCategory::UA_LOGCATEGORY_SERVER => LOG_CATEGORY_SERVER,
        UA_LogCategory::UA_LOGCATEGORY_CLIENT => LOG_CATEGORY_CLIENT,
        UA_LogCategory::UA_LOGCATEGORY_USERLAND => LOG_CATEGORY_USERLAND,
        UA_LogCategory::UA_LOGCATEGORY_SECURITYPOLICY => LOG_CATEGORY_SECURITYPOLICY,
        UA_LogCategory::UA_LOGCATEGORY_EVENTLOOP => LOG_CATEGORY_EVENTLOOP,
        UA_LogCategory::UA_LOGCATEGORY_PUBSUB => LOG_CATEGORY_PUBSUB,
        UA_LogCategory::UA_LOGCATEGORY_DISCOVERY => LOG_CATEGORY_DISCOVERY,
        _ => LOG_CATEGORY_UNKNOWN,
    }
}
