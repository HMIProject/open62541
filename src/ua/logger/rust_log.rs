use std::{
    ffi::{c_char, c_void},
    ptr,
};

use log::Level;
use open62541_sys::{UA_LogCategory, UA_LogLevel, UA_Logger, UA_String_vformat};

use crate::{ua, DataType as _, Error, Result};

// This matches the crate name.
const LOG_TARGET: &str = "open62541_sys";

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
            Ok(ref msg) => msg.as_str().unwrap_or("Invalid log message"),
            Err(_) => "Unknown log message",
        };

        let category = match category {
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
        };

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
