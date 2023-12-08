use std::{
    ffi::c_void,
    ptr,
    sync::{Arc, Mutex, Weak},
};

use futures::{channel::oneshot, stream, Stream};
use log::debug;
use open62541_sys::{
    UA_Client, UA_Client_DataChangeNotificationCallback, UA_Client_DeleteMonitoredItemCallback,
    UA_Client_MonitoredItems_createDataChanges_async, UA_Client_MonitoredItems_delete_async,
    UA_CreateMonitoredItemsResponse, UA_DataValue, UA_StatusCode, UA_UInt32, UA_STATUSCODE_GOOD,
};
use tokio::sync::watch;

use crate::{ua, CallbackMut, CallbackOnce, Error, MonitoredItemId, SubscriptionId};

pub struct AsyncMonitoredItem {
    client: Weak<Mutex<ua::Client>>,
    monitored_item_id: MonitoredItemId,
    rx: watch::Receiver<Option<ua::DataValue>>,
}

impl AsyncMonitoredItem {
    pub(crate) async fn new(
        client: Arc<Mutex<ua::Client>>,
        subscription_id: SubscriptionId,
        node_id: ua::NodeId,
    ) -> Result<Self, Error> {
        let request = ua::CreateMonitoredItemsRequest::init()
            .with_subscription_id(subscription_id)
            .with_items_to_create(&[ua::MonitoredItemCreateRequest::init_node_id(node_id)]);

        let (response, rx) = create_monitored_items(client.clone(), request).await?;

        // PANIC: We expect exactly one result for the monitored item we requested above.
        let monitored_item_id = *response.monitored_item_ids().unwrap().get(0).unwrap();

        Ok(AsyncMonitoredItem {
            client: Arc::downgrade(&client),
            monitored_item_id,
            rx,
        })
    }

    /// Waits for next value from server.
    ///
    /// This waits for the next value received for this monitored item. Returns `None` when item has
    /// been closed and no more updates will be received.
    pub async fn next(&mut self) -> Option<ua::DataValue> {
        // Wait for next change of the underlying value. This always skips the initial `None` value,
        // so the only way to return `None` from this function is through `ok()` here (i.e. when the
        // channel has been closed).
        self.rx.changed().await.ok()?;

        let value = self.rx.borrow().clone();
        debug_assert!(value.is_some(), "should skip initial `None` value");
        value
    }

    /// Turns monitored item into stream.
    ///
    /// The stream will emit all value updates as they are being received. If the client disconnects
    /// or the corresponding subscription is deleted, the stream is closed.
    pub fn into_stream(self) -> impl Stream<Item = ua::DataValue> {
        stream::unfold(self, move |mut this| async move {
            this.next().await.map(|value| (value, this))
        })
    }
}

impl Drop for AsyncMonitoredItem {
    fn drop(&mut self) {
        let Some(client) = self.client.upgrade() else {
            return;
        };

        let request = ua::DeleteMonitoredItemsRequest::init()
            .with_monitored_item_ids(&[self.monitored_item_id]);

        delete_monitored_items(&client, request);
    }
}

async fn create_monitored_items(
    client: Arc<Mutex<ua::Client>>,
    request: ua::CreateMonitoredItemsRequest,
) -> Result<
    (
        ua::CreateMonitoredItemsResponse,
        watch::Receiver<Option<ua::DataValue>>,
    ),
    Error,
> {
    type St = CallbackMut<Option<ua::DataValue>>;
    type Cb = CallbackOnce<Result<ua::CreateMonitoredItemsResponse, UA_StatusCode>>;

    unsafe extern "C" fn notification_callback_c(
        _client: *mut UA_Client,
        _sub_id: UA_UInt32,
        _sub_context: *mut c_void,
        _mon_id: UA_UInt32,
        mon_context: *mut c_void,
        value: *mut UA_DataValue,
    ) {
        debug!("DataChangeNotificationCallback() was called");

        // PANIC: We expect pointer to be valid when called.
        let value = value.as_ref().expect("value is set");
        let value = ua::DataValue::from_ref(value);
        St::notify(mon_context, Some(value));
    }

    unsafe extern "C" fn delete_callback_c(
        _client: *mut UA_Client,
        _sub_id: UA_UInt32,
        _sub_context: *mut c_void,
        _mon_id: UA_UInt32,
        mon_context: *mut c_void,
    ) {
        debug!("DeleteMonitoredItemCallback() was called");

        St::delete(mon_context);
    }

    unsafe extern "C" fn callback_c(
        _client: *mut UA_Client,
        userdata: *mut c_void,
        _request_id: UA_UInt32,
        response: *mut c_void,
    ) {
        debug!("MonitoredItems_createDataChanges() completed");

        let response = response.cast::<UA_CreateMonitoredItemsResponse>();
        let status = (*response).responseHeader.serviceResult;
        let result = if status == UA_STATUSCODE_GOOD {
            // PANIC: We expect pointer to be valid when good.
            let response = response.as_ref().expect("response is set");
            Ok(ua::CreateMonitoredItemsResponse::from_ref(response))
        } else {
            Err(status)
        };
        Cb::execute(userdata, result);
    }

    let (tx, rx) = oneshot::channel::<Result<ua::CreateMonitoredItemsResponse, Error>>();
    // TODO: Think about appropriate buffer size or let the caller decide.
    let (st_tx, st_rx) = watch::channel::<Option<ua::DataValue>>(None);

    let callback = |result: Result<ua::CreateMonitoredItemsResponse, _>| {
        // We always send a result back via `tx` (in fact, `rx.await` below expects this). We do not
        // care if that succeeds though: the receiver might already have gone out of scope (when its
        // future has been canceled) and we must not panic in FFI callbacks.
        let _unused = tx.send(result.map_err(Error::new));
    };

    let mut notification_callbacks: Vec<UA_Client_DataChangeNotificationCallback> =
        vec![Some(notification_callback_c)];
    let mut delete_callbacks: Vec<UA_Client_DeleteMonitoredItemCallback> =
        vec![Some(delete_callback_c)];
    let mut contexts: Vec<*mut c_void> = vec![St::prepare(st_tx)];

    let result = {
        let mut client = client.lock().unwrap();

        debug!("Calling MonitoredItems_createDataChanges()");

        unsafe {
            UA_Client_MonitoredItems_createDataChanges_async(
                client.as_mut_ptr(),
                request.into_inner(),
                contexts.as_mut_ptr(),
                notification_callbacks.as_mut_ptr(),
                delete_callbacks.as_mut_ptr(),
                Some(callback_c),
                Cb::prepare(callback),
                ptr::null_mut(),
            )
        }
    };
    if result != UA_STATUSCODE_GOOD {
        return Err(Error::new(result));
    }

    // PANIC: When `callback` is called (which owns `tx`), we always call `tx.send()`. So the sender
    // is only dropped after placing a value into the channel and `rx.await` always finds this value
    // there.
    rx.await.unwrap().map(|response| (response, st_rx))
}

fn delete_monitored_items(
    client: &Arc<Mutex<ua::Client>>,
    request: ua::DeleteMonitoredItemsRequest,
) {
    unsafe extern "C" fn callback_c(
        _client: *mut UA_Client,
        _userdata: *mut c_void,
        _request_id: UA_UInt32,
        _response: *mut c_void,
    ) {
        debug!("MonitoredItems_delete() completed");

        // Nothing to do here.
    }

    let _unused = {
        let Ok(mut client) = client.lock() else {
            return;
        };

        debug!("Calling MonitoredItems_delete()");

        unsafe {
            UA_Client_MonitoredItems_delete_async(
                client.as_mut_ptr(),
                request.into_inner(),
                // This must be set (despite the `Option` type). The internal handler in `open62541`
                // calls our callback unconditionally (as opposed to other service functions where a
                // handler may be left unset if not required).
                Some(callback_c),
                ptr::null_mut(),
                ptr::null_mut(),
            )
        }
    };
}
