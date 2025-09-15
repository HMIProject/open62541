use std::{ffi::c_void, mem, ptr};

use futures_channel::oneshot;
use open62541_sys::{
    UA_Client, UA_Client_DataChangeNotificationCallback, UA_Client_DeleteMonitoredItemCallback,
    UA_Client_MonitoredItems_createDataChanges_async, UA_CreateMonitoredItemsResponse,
    UA_DataValue, UA_UInt32, UA_Variant,
};

use crate::{
    ua, CallbackMut, CallbackOnce, DataType as _, Error, MonitoredItemKind, MonitoredItemValue,
    Result,
};

type CbResponse =
    CallbackOnce<std::result::Result<ua::CreateMonitoredItemsResponse, ua::StatusCode>>;
type CbNotification = CallbackMut<MonitoredItemValue>;

// Wrapper type so that we can mark `*mut c_void` for callbacks as safe to send.
#[repr(transparent)]
struct Context(*mut c_void);

// SAFETY: As long as payload is `Send`, wrapper is `Send`.
unsafe impl Send for Context where CbNotification: Send + Sync {}

/// Creates monitored items.
///
/// Notifications for monitored are handled by callback closures. The callback
/// closures are created by invoking `create_value_callback_fn` repeatedly while
/// preparing the items to create for the request. The argument of `create_value_callback_fn`
/// is an `index` that starts at 0 and is incremented by 1 consecutively, i.e. by enumerating
/// the items to be created.
//
// TODO: How to pass `&ua::MonitoredItemCreateRequest` as a second argument to
// `create_value_callback_fn`? `impl for<'a> FnMut(usize, &'a ua::MonitoredItemCreateRequest) -> F`
// doesn't work.
// See also: <https://rust-lang.github.io/rfcs/3216-closure-lifetime-binder.html>
pub(super) async fn call<K: MonitoredItemKind, F>(
    client: &ua::Client,
    request: &ua::CreateMonitoredItemsRequest,
    mut create_value_callback_fn: impl FnMut(usize) -> F,
) -> Result<ua::CreateMonitoredItemsResponse>
where
    F: FnMut(K::Value) + 'static,
{
    let (tx, rx) = oneshot::channel::<Result<ua::CreateMonitoredItemsResponse>>();

    let response_callback =
        move |result: std::result::Result<ua::CreateMonitoredItemsResponse, _>| {
            // We always send a result back via `tx` (in fact, `rx.await` below expects this). We do not
            // care if that succeeds though: the receiver might already have gone out of scope (when its
            // future has been cancelled) and we must not panic in FFI callbacks.
            let _unused = tx.send(result.map_err(Error::new));
        };

    let items_to_create = request.items_to_create().unwrap_or_default();

    let mut notification_callbacks: Vec<UA_Client_DataChangeNotificationCallback> =
        Vec::with_capacity(items_to_create.len());
    let mut delete_notification_callbacks: Vec<UA_Client_DeleteMonitoredItemCallback> =
        Vec::with_capacity(items_to_create.len());
    let mut contexts = Vec::with_capacity(items_to_create.len());

    for (item_index, item_to_create) in items_to_create.iter().enumerate() {
        // `open62541` requires one set of notification/delete callback and context per monitored
        // item in the request.
        let notification_callback = NotificationCallback::for_request(item_to_create);
        let delete_notification_callback: UA_Client_DeleteMonitoredItemCallback =
            Some(delete_notification_callback_c);

        // TODO: let value_callback = create_value_callback_fn(item_index, item_to_create);
        let mut value_callback: F = create_value_callback_fn(item_index);
        let map_value_callback = move |value| {
            // TODO: How to get rid of the intermediate, internal mapping into `MonitoredItemValue`?
            value_callback(K::map_value(value));
        };
        let context = Context(CbNotification::prepare(map_value_callback));

        // SAFETY: This cast is possible because `UA_Client_MonitoredItems_createDataChanges_async`
        // internally casts the function pointer back to the appropriate type before calling (union
        // type of attribute `handler` in `UA_Client_MonitoredItem`).
        notification_callbacks.push(Some(unsafe { notification_callback.into_data_change() }));
        delete_notification_callbacks.push(delete_notification_callback);
        contexts.push(context);
    }

    let status_code = ua::StatusCode::new({
        log::debug!(
            "Calling MonitoredItems_createDataChanges(), count={}",
            contexts.len()
        );

        // SAFETY: `UA_Client_MonitoredItems_createDataChanges_async()` expects the request passed
        // by value but does not take ownership.
        let request = unsafe { ua::CreateMonitoredItemsRequest::to_raw_copy(request) };

        unsafe {
            // This handles both data change and event notifications as determined by the monitored
            // attribute ID, delegating to `createDataChanges_async()` in both cases. We must still
            // make sure to pass the expected callback function in `notification_callbacks` above.
            UA_Client_MonitoredItems_createDataChanges_async(
                // SAFETY: Cast to `mut` pointer, function is marked `UA_THREADSAFE`.
                client.as_ptr().cast_mut(),
                request,
                contexts.as_mut_ptr().cast::<*mut c_void>(),
                notification_callbacks.as_mut_ptr(),
                delete_notification_callbacks.as_mut_ptr(),
                Some(response_callback_c),
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

enum NotificationCallback {
    DataChange,
    Event,
}

impl NotificationCallback {
    fn for_request(request: &ua::MonitoredItemCreateRequest) -> Self {
        if request.attribute_id() == ua::AttributeId::EVENTNOTIFIER {
            Self::Event
        } else {
            Self::DataChange
        }
    }

    /// Provides callback function for C call.
    ///
    /// # Safety
    ///
    /// This always returns a function pointer for [`UA_Client_DataChangeNotificationCallback`], for
    /// both data change _and_ event callbacks. Care must be taken to only pass the expected handler
    /// to the corresponding [`ua::MonitoredItemCreateRequest`], depending on the attribute ID.
    unsafe fn into_data_change(self) -> DataChangeNotificationCallbackC {
        match self {
            Self::DataChange => data_change_notification_callback_c,

            // This is rather unfortunate. Since we cannot call `createDataChanges_async()` directly
            // (it is not exported by open62541), we must use one of the two wrapper functions, i.e.
            // `UA_Client_MonitoredItems_create[DataChanges|Events]_async()`, instead. These wrapper
            // functions only adjust the types in the function signature and add a mutex lock. Thus,
            // apart from the fact that open62541 does some `void` pointer magic, the transmute here
            // is safe (at least not more unsafe/unportable than the underlying C code already is).
            Self::Event => unsafe {
                mem::transmute::<EventNotificationCallbackC, DataChangeNotificationCallbackC>(
                    event_notification_callback_c,
                )
            },
        }
    }
}

type DataChangeNotificationCallbackC = unsafe extern "C" fn(
    client: *mut UA_Client,
    sub_id: UA_UInt32,
    sub_context: *mut c_void,
    mon_id: UA_UInt32,
    mon_context: *mut c_void,
    value: *mut UA_DataValue,
);

unsafe extern "C" fn data_change_notification_callback_c(
    _client: *mut UA_Client,
    _sub_id: UA_UInt32,
    _sub_context: *mut c_void,
    _mon_id: UA_UInt32,
    mon_context: *mut c_void,
    value: *mut UA_DataValue,
) {
    log::debug!("DataChangeNotificationCallback() was called");

    // SAFETY: Incoming pointer is valid for access.
    // PANIC: We expect pointer to be valid when called.
    let value = unsafe { value.as_ref() }.expect("value should be set");
    let value = ua::DataValue::clone_raw(value);

    // SAFETY: `mon_context` is result of `CbNotification::prepare()` and is used only before `delete()`.
    unsafe {
        CbNotification::execute(mon_context, MonitoredItemValue::data_change(value));
    }
}

type EventNotificationCallbackC = unsafe extern "C" fn(
    client: *mut UA_Client,
    sub_id: UA_UInt32,
    sub_context: *mut c_void,
    mon_id: UA_UInt32,
    mon_context: *mut c_void,
    n_event_fields: usize,
    event_fields: *mut UA_Variant,
);

unsafe extern "C" fn event_notification_callback_c(
    _client: *mut UA_Client,
    _sub_id: UA_UInt32,
    _sub_context: *mut c_void,
    _mon_id: UA_UInt32,
    mon_context: *mut c_void,
    n_event_fields: usize,
    event_fields: *mut UA_Variant,
) {
    log::debug!("EventNotificationCallback() was called");

    // PANIC: We expect pointer to be valid when called.
    let fields = ua::Array::from_raw_parts(n_event_fields, event_fields)
        .expect("event fields should be set");

    // SAFETY: `mon_context` is result of `CbNotification::prepare()` and is used only before `delete()`.
    unsafe {
        CbNotification::execute(mon_context, MonitoredItemValue::event(fields));
    }
}

unsafe extern "C" fn delete_notification_callback_c(
    _client: *mut UA_Client,
    _sub_id: UA_UInt32,
    _sub_context: *mut c_void,
    _mon_id: UA_UInt32,
    mon_context: *mut c_void,
) {
    log::debug!("DeleteMonitoredItemCallback() was called");

    // SAFETY: `mon_context` is result of `CbNotification::prepare()` and is used only before `delete()`.
    unsafe {
        CbNotification::delete(mon_context);
    }
}

unsafe extern "C" fn response_callback_c(
    _client: *mut UA_Client,
    userdata: *mut c_void,
    _request_id: UA_UInt32,
    response: *mut c_void,
) {
    log::debug!("MonitoredItems_createDataChanges() completed");

    let response = response.cast::<UA_CreateMonitoredItemsResponse>();
    // SAFETY: Incoming pointer is valid for access.
    // PANIC: We expect pointer to be valid when good.
    let response = unsafe { response.as_ref() }.expect("response should be set");
    let status_code = ua::StatusCode::new(response.responseHeader.serviceResult);

    let result = if status_code.is_good() {
        Ok(ua::CreateMonitoredItemsResponse::clone_raw(response))
    } else {
        Err(status_code)
    };

    // SAFETY: `userdata` is the result of `CbResponse::prepare()` and is used only once.
    unsafe {
        CbResponse::execute(userdata, result);
    }
}
