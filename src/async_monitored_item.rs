use std::{
    ffi::c_void,
    pin::Pin,
    ptr,
    sync::{Arc, Mutex, Weak},
    task::{self, Poll},
};

use futures_channel::oneshot;
use futures_core::Stream;
use futures_util::stream;
use open62541_sys::{
    UA_Client, UA_Client_DataChangeNotificationCallback, UA_Client_DeleteMonitoredItemCallback,
    UA_Client_MonitoredItems_createDataChanges_async, UA_Client_MonitoredItems_delete_async,
    UA_CreateMonitoredItemsResponse, UA_DataValue, UA_UInt32,
};
use tokio::sync::mpsc;

use crate::{ua, CallbackOnce, CallbackStream, DataType as _, Error, Result};

/// Monitored item (with asynchronous API).
pub struct AsyncMonitoredItem {
    client: Weak<Mutex<ua::Client>>,
    monitored_item_id: ua::MonitoredItemId,
    rx: mpsc::Receiver<ua::DataValue>,
}

impl AsyncMonitoredItem {
    pub(crate) async fn new(
        client: &Arc<Mutex<ua::Client>>,
        subscription_id: &ua::SubscriptionId,
        node_id: &ua::NodeId,
    ) -> Result<Self> {
        let create_request = ua::MonitoredItemCreateRequest::default().with_node_id(node_id);

        let request = ua::CreateMonitoredItemsRequest::init()
            .with_subscription_id(subscription_id)
            .with_items_to_create(&[create_request]);

        let (response, rx) = create_monitored_items(client, &request).await?;

        // PANIC: We expect exactly one result for the monitored item we requested above.
        let monitored_item_id = *response.monitored_item_ids().unwrap().first().unwrap();

        Ok(AsyncMonitoredItem {
            client: Arc::downgrade(client),
            monitored_item_id,
            rx,
        })
    }

    /// Waits for next value from server.
    ///
    /// This waits for the next value received for this monitored item. Returns `None` when item has
    /// been closed and no more updates will be received.
    pub async fn next(&mut self) -> Option<ua::DataValue> {
        // This mirrors `<Self as Stream>::poll_next()` but does not require `self` to be pinned.
        self.rx.recv().await
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

        delete_monitored_items(&client, &request);
    }
}

impl Stream for AsyncMonitoredItem {
    type Item = ua::DataValue;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Option<Self::Item>> {
        // This mirrors `AsyncMonitoredItem::next()` and implements the `Stream` trait.
        self.rx.poll_recv(cx)
    }
}

/// Maximum number of buffered values.
const MONITORED_ITEM_BUFFER_SIZE: usize = 3;

async fn create_monitored_items(
    client: &Mutex<ua::Client>,
    request: &ua::CreateMonitoredItemsRequest,
) -> Result<(
    ua::CreateMonitoredItemsResponse,
    mpsc::Receiver<ua::DataValue>,
)> {
    type St = CallbackStream<ua::DataValue>;
    type Cb = CallbackOnce<std::result::Result<ua::CreateMonitoredItemsResponse, ua::StatusCode>>;

    // Wrapper type so that we can mark `*mut c_void` for callbacks as safe to send. Otherwise, this
    // would make any closure that uses `AsyncMonitoredItem::new()` not `Send`.
    #[repr(transparent)]
    struct Context(*mut c_void);
    // SAFETY: As long as the payload is `Send`, context is also `Send`.
    unsafe impl Send for Context where St: Send + Sync {}

    unsafe extern "C" fn notification_callback_c(
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

        // SAFETY: `userdata` is the result of `St::prepare()` and is used only before `delete()`.
        unsafe {
            St::notify(mon_context, value);
        }
    }

    unsafe extern "C" fn delete_callback_c(
        _client: *mut UA_Client,
        _sub_id: UA_UInt32,
        _sub_context: *mut c_void,
        _mon_id: UA_UInt32,
        mon_context: *mut c_void,
    ) {
        log::debug!("DeleteMonitoredItemCallback() was called");

        // SAFETY: `userdata` is the result of `St::prepare()` and is deleted only once.
        unsafe {
            St::delete(mon_context);
        }
    }

    unsafe extern "C" fn callback_c(
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

        // SAFETY: `userdata` is the result of `Cb::prepare()` and is used only once.
        unsafe {
            Cb::execute(userdata, result);
        }
    }

    let (tx, rx) = oneshot::channel::<Result<ua::CreateMonitoredItemsResponse>>();
    // TODO: Think about appropriate buffer size or let the caller decide.
    let (st_tx, st_rx) = mpsc::channel::<ua::DataValue>(MONITORED_ITEM_BUFFER_SIZE);

    let callback = |result: std::result::Result<ua::CreateMonitoredItemsResponse, _>| {
        // We always send a result back via `tx` (in fact, `rx.await` below expects this). We do not
        // care if that succeeds though: the receiver might already have gone out of scope (when its
        // future has been canceled) and we must not panic in FFI callbacks.
        let _unused = tx.send(result.map_err(Error::new));
    };

    let mut notification_callbacks: Vec<UA_Client_DataChangeNotificationCallback> =
        vec![Some(notification_callback_c)];
    let mut delete_callbacks: Vec<UA_Client_DeleteMonitoredItemCallback> =
        vec![Some(delete_callback_c)];
    let mut contexts = vec![Context(St::prepare(st_tx))];

    let status_code = ua::StatusCode::new({
        let Ok(mut client) = client.lock() else {
            return Err(Error::internal("should be able to lock client"));
        };

        log::debug!(
            "Calling MonitoredItems_createDataChanges(), count={}",
            contexts.len()
        );

        // SAFETY: `UA_Client_MonitoredItems_createDataChanges_async()` expects the request passed
        // by value but does not take ownership.
        let request = unsafe { ua::CreateMonitoredItemsRequest::to_raw_copy(request) };

        unsafe {
            UA_Client_MonitoredItems_createDataChanges_async(
                client.as_mut_ptr(),
                request,
                contexts.as_mut_ptr().cast::<*mut c_void>(),
                notification_callbacks.as_mut_ptr(),
                delete_callbacks.as_mut_ptr(),
                Some(callback_c),
                Cb::prepare(callback),
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
        .map(|response| (response, st_rx))
}

fn delete_monitored_items(client: &Mutex<ua::Client>, request: &ua::DeleteMonitoredItemsRequest) {
    unsafe extern "C" fn callback_c(
        _client: *mut UA_Client,
        _userdata: *mut c_void,
        _request_id: UA_UInt32,
        _response: *mut c_void,
    ) {
        log::debug!("MonitoredItems_delete() completed");

        // Nothing to do here.
    }

    let _unused = {
        let Ok(mut client) = client.lock() else {
            return;
        };

        log::debug!("Calling MonitoredItems_delete()");

        // SAFETY: `UA_Client_MonitoredItems_delete_async()` expects the request passed by value but
        // does not take ownership.
        let request = unsafe { ua::DeleteMonitoredItemsRequest::to_raw_copy(request) };

        unsafe {
            UA_Client_MonitoredItems_delete_async(
                client.as_mut_ptr(),
                request,
                // This must be set despite the `Option` type. The internal handler in `open62541`
                // calls our callback unconditionally (in contrast to other service functions where
                // a handler may be left unset when it is not required).
                Some(callback_c),
                ptr::null_mut(),
                ptr::null_mut(),
            )
        }
    };
}
