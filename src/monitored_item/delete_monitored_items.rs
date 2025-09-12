use std::{ffi::c_void, ptr};

use futures_channel::oneshot;
use open62541_sys::{
    UA_Client, UA_Client_MonitoredItems_delete_async, UA_DeleteMonitoredItemsResponse, UA_UInt32,
};

use crate::{ua, CallbackOnce, DataType as _, Error, Result};

type CbResponse =
    CallbackOnce<std::result::Result<ua::DeleteMonitoredItemsResponse, ua::StatusCode>>;

/// Sends request and awaits response.
// TODO: Reduce visibility to pub(super).
pub(super) async fn call(
    client: &ua::Client,
    request: &ua::DeleteMonitoredItemsRequest,
) -> Result<ua::DeleteMonitoredItemsResponse> {
    let (tx, rx) = oneshot::channel::<Result<ua::DeleteMonitoredItemsResponse>>();

    let status_code = ua::StatusCode::new({
        // SAFETY: `UA_Client_MonitoredItems_delete_async()` expects the request passed by value but
        // does not take ownership.
        let request = unsafe { ua::DeleteMonitoredItemsRequest::to_raw_copy(request) };

        let response_callback =
            |result: std::result::Result<ua::DeleteMonitoredItemsResponse, _>| {
                // We always send a result back via `tx` (in fact, `rx.await` below expects this). We do not
                // care if that succeeds though: the receiver might already have gone out of scope (when its
                // future has been cancelled) and we must not panic in FFI callbacks.
                let _unused = tx.send(result.map_err(Error::new));
            };

        log::debug!("Calling MonitoredItems_delete_async()");
        unsafe {
            UA_Client_MonitoredItems_delete_async(
                // SAFETY: Cast to `mut` pointer, function is marked `UA_THREADSAFE`.
                client.as_ptr().cast_mut(),
                request,
                Some(callback_execute_response_c),
                CbResponse::prepare(response_callback),
                ptr::null_mut(),
            )
        }
    });
    Error::verify_good(&status_code)?;

    // PANIC: When `callback` is called (which owns `tx`), we always call `tx.send()`. So the sender
    // is only dropped after placing a value into the channel and `rx.await` always finds this value
    // there.
    rx.await
        .unwrap_or(Err(Error::internal("callback should send result")))
}

unsafe extern "C" fn callback_execute_response_c(
    _client: *mut UA_Client,
    userdata: *mut c_void,
    _request_id: UA_UInt32,
    response: *mut c_void,
) {
    log::debug!("MonitoredItems_delete_async() completed");

    let response = response.cast::<UA_DeleteMonitoredItemsResponse>();
    // SAFETY: Incoming pointer is valid for access.
    // PANIC: We expect pointer to be valid when good.
    let response = unsafe { response.as_ref() }.expect("response should be set");
    let status_code = ua::StatusCode::new(response.responseHeader.serviceResult);

    let result = if status_code.is_good() {
        Ok(ua::DeleteMonitoredItemsResponse::clone_raw(response))
    } else {
        Err(status_code)
    };

    // SAFETY: `userdata` is the result of `CbResponse::prepare()` and is used only once.
    unsafe {
        CbResponse::execute(userdata, result);
    }
}

/// Sends request and returns.
///
/// Synchronous fire-and-forget variant of [`call()`] that returns immediately
/// after sending the request and does not await the response.
///
/// Only supposed to be used in implementations of `Drop`.
pub(super) fn send_request(
    client: &ua::Client,
    request: &ua::DeleteMonitoredItemsRequest,
) -> Result<()> {
    let status_code = ua::StatusCode::new({
        // SAFETY: `UA_Client_MonitoredItems_delete_async()` expects the request passed by value but
        // does not take ownership.
        let request = unsafe { ua::DeleteMonitoredItemsRequest::to_raw_copy(request) };

        log::debug!("Calling MonitoredItems_delete_async()");
        unsafe {
            UA_Client_MonitoredItems_delete_async(
                // SAFETY: Cast to `mut` pointer, function is marked `UA_THREADSAFE`.
                client.as_ptr().cast_mut(),
                request,
                Some(callback_log_response_c),
                ptr::null_mut(),
                ptr::null_mut(),
            )
        }
    });
    Error::verify_good(&status_code)?;

    Ok(())
}

unsafe extern "C" fn callback_log_response_c(
    _client: *mut UA_Client,
    _userdata: *mut c_void,
    _request_id: UA_UInt32,
    response: *mut c_void,
) {
    log::debug!("MonitoredItems_delete_async() completed");

    let response = response.cast::<UA_DeleteMonitoredItemsResponse>();
    // SAFETY: Incoming pointer is valid for access.
    // PANIC: We expect pointer to be valid when good.
    let response = unsafe { response.as_ref() }.expect("response should be set");
    let status_code = ua::StatusCode::new(response.responseHeader.serviceResult);

    if let Err(error) = Error::verify_good(&status_code) {
        log::warn!("Error in response when deleting monitored items: {error}");
    }
}
