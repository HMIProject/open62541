use std::{ffi::c_void, ptr};

use open62541_sys::{
    UA_Client, UA_Client_MonitoredItems_delete_async, UA_DeleteMonitoredItemsResponse, UA_UInt32,
};

use crate::{ua, DataType as _, Error};

pub(super) fn call(client: &ua::Client, request: &ua::DeleteMonitoredItemsRequest) {
    unsafe extern "C" fn callback_c(
        _client: *mut UA_Client,
        _userdata: *mut c_void,
        _request_id: UA_UInt32,
        response: *mut c_void,
    ) {
        log::debug!("MonitoredItems_delete() completed");

        let response = response.cast::<UA_DeleteMonitoredItemsResponse>();
        // SAFETY: Incoming pointer is valid for access.
        // PANIC: We expect pointer to be valid when good.
        let response = unsafe { response.as_ref() }.expect("response should be set");
        let status_code = ua::StatusCode::new(response.responseHeader.serviceResult);

        if let Err(error) = Error::verify_good(&status_code) {
            log::warn!("Error in response when deleting monitored items: {error}");
        }
    }

    let status_code = ua::StatusCode::new({
        log::debug!("Calling MonitoredItems_delete()");

        // SAFETY: `UA_Client_MonitoredItems_delete_async()` expects the request passed by value but
        // does not take ownership.
        let request = unsafe { ua::DeleteMonitoredItemsRequest::to_raw_copy(request) };

        unsafe {
            UA_Client_MonitoredItems_delete_async(
                // SAFETY: Cast to `mut` pointer, function is marked `UA_THREADSAFE`.
                client.as_ptr().cast_mut(),
                request,
                Some(callback_c),
                ptr::null_mut(),
                ptr::null_mut(),
            )
        }
    });
    if let Err(error) = Error::verify_good(&status_code) {
        log::warn!("Error in request when deleting monitored items: {error}");
    }
}
