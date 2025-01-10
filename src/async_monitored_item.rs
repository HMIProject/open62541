use std::{
    ffi::c_void,
    pin::Pin,
    ptr,
    sync::{Arc, Weak},
    task::{self, Poll},
    time::Duration,
};

use futures_channel::oneshot;
use futures_core::Stream;
use futures_util::stream;
use open62541_sys::{
    UA_Client, UA_Client_DataChangeNotificationCallback, UA_Client_DeleteMonitoredItemCallback,
    UA_Client_MonitoredItems_createDataChanges_async, UA_Client_MonitoredItems_delete_async,
    UA_CreateMonitoredItemsResponse, UA_DataValue, UA_DeleteMonitoredItemsResponse, UA_UInt32,
};
use tokio::sync::mpsc;

use crate::{ua, AsyncSubscription, CallbackOnce, CallbackStream, DataType as _, Error, Result};

#[derive(Debug)]
pub struct MonitoredItemBuilder {
    node_ids: Vec<ua::NodeId>,
    monitoring_mode: Option<ua::MonitoringMode>,
    #[allow(clippy::option_option)]
    sampling_interval: Option<Option<Duration>>,
    queue_size: Option<u32>,
    discard_oldest: Option<bool>,
}

impl MonitoredItemBuilder {
    pub fn new(node_ids: impl IntoIterator<Item = ua::NodeId>) -> Self {
        Self {
            node_ids: node_ids.into_iter().collect(),
            monitoring_mode: None,
            sampling_interval: None,
            queue_size: None,
            discard_oldest: None,
        }
    }

    /// Sets monitoring mode.
    ///
    /// See [`ua::MonitoredItemCreateRequest::with_monitoring_mode()`].
    #[must_use]
    pub fn monitoring_mode(mut self, monitoring_mode: ua::MonitoringMode) -> Self {
        self.monitoring_mode = Some(monitoring_mode);
        self
    }

    /// Sets sampling interval.
    ///
    /// See [`ua::MonitoringParameters::with_sampling_interval()`].
    #[must_use]
    pub const fn sampling_interval(mut self, sampling_interval: Option<Duration>) -> Self {
        self.sampling_interval = Some(sampling_interval);
        self
    }

    /// Set requested size of the monitored item queue.
    ///
    /// See [`ua::MonitoringParameters::with_queue_size()`].
    #[must_use]
    pub const fn queue_size(mut self, queue_size: u32) -> Self {
        self.queue_size = Some(queue_size);
        self
    }

    /// Set discard policy.
    ///
    /// See [`ua::MonitoringParameters::with_discard_oldest()`].
    #[must_use]
    pub const fn discard_oldest(mut self, discard_oldest: bool) -> Self {
        self.discard_oldest = Some(discard_oldest);
        self
    }

    /// Creates monitored items.
    ///
    /// This creates one or more new monitored items.
    ///
    /// # Errors
    ///
    /// This fails when one of the nodes does not exist.
    pub async fn create(self, subscription: &AsyncSubscription) -> Result<Vec<AsyncMonitoredItem>> {
        let Some(client) = &subscription.client().upgrade() else {
            return Err(Error::internal("client should not be dropped"));
        };
        let subscription_id = subscription.subscription_id();

        let (response, rxs) =
            create_monitored_items(client, &self.into_request(subscription_id)).await?;

        let Some(monitored_item_ids) = response.monitored_item_ids() else {
            return Err(Error::internal("expected monitored item IDs"));
        };

        // PANIC: We expect exactly one result for each monitored item we requested above.
        debug_assert_eq!(monitored_item_ids.len(), rxs.len());

        Ok(monitored_item_ids
            .into_iter()
            .zip(rxs)
            .map(|(monitored_item_id, rx)| AsyncMonitoredItem {
                client: Arc::downgrade(client),
                subscription_id,
                monitored_item_id,
                rx,
            })
            .collect())
    }

    fn into_request(self, subscription_id: ua::SubscriptionId) -> ua::CreateMonitoredItemsRequest {
        let Self {
            node_ids,
            monitoring_mode,
            sampling_interval,
            queue_size,
            discard_oldest,
        } = self;

        let items_to_create = node_ids
            .into_iter()
            .map(|node_id| {
                let mut request = ua::MonitoredItemCreateRequest::default().with_node_id(&node_id);

                if let Some(monitoring_mode) = monitoring_mode.as_ref() {
                    request = request.with_monitoring_mode(monitoring_mode);
                }
                if let Some(&sampling_interval) = sampling_interval.as_ref() {
                    request = request.with_sampling_interval(sampling_interval);
                }
                if let Some(&queue_size) = queue_size.as_ref() {
                    request = request.with_queue_size(queue_size);
                }
                if let Some(&discard_oldest) = discard_oldest.as_ref() {
                    request = request.with_discard_oldest(discard_oldest);
                }

                request
            })
            .collect::<Vec<_>>();

        ua::CreateMonitoredItemsRequest::init()
            .with_subscription_id(subscription_id)
            .with_items_to_create(&items_to_create)
    }
}

/// Monitored item (with asynchronous API).
#[derive(Debug)]
pub struct AsyncMonitoredItem {
    client: Weak<ua::Client>,
    subscription_id: ua::SubscriptionId,
    monitored_item_id: ua::MonitoredItemId,
    rx: mpsc::Receiver<ua::DataValue>,
}

impl AsyncMonitoredItem {
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
    pub fn into_stream(self) -> impl Stream<Item = ua::DataValue> + Send + Sync + 'static {
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
            .with_subscription_id(self.subscription_id)
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
    client: &ua::Client,
    request: &ua::CreateMonitoredItemsRequest,
) -> Result<(
    ua::CreateMonitoredItemsResponse,
    Vec<mpsc::Receiver<ua::DataValue>>,
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

    let callback = |result: std::result::Result<ua::CreateMonitoredItemsResponse, _>| {
        // We always send a result back via `tx` (in fact, `rx.await` below expects this). We do not
        // care if that succeeds though: the receiver might already have gone out of scope (when its
        // future has been cancelled) and we must not panic in FFI callbacks.
        let _unused = tx.send(result.map_err(Error::new));
    };

    let items_to_create = request
        .items_to_create()
        .map_or(0, <[ua::MonitoredItemCreateRequest]>::len);

    let mut notification_callbacks: Vec<UA_Client_DataChangeNotificationCallback> =
        Vec::with_capacity(items_to_create);
    let mut delete_callbacks: Vec<UA_Client_DeleteMonitoredItemCallback> =
        Vec::with_capacity(items_to_create);
    let mut contexts = Vec::with_capacity(items_to_create);
    let mut st_rxs = Vec::with_capacity(items_to_create);

    for _ in 0..items_to_create {
        // TODO: Think about appropriate buffer size or let the caller decide.
        let (st_tx, st_rx) = mpsc::channel::<ua::DataValue>(MONITORED_ITEM_BUFFER_SIZE);

        // `open62541` requires one set of notification/delete callback and context per monitor item
        // in the request.
        let notification_callback: UA_Client_DataChangeNotificationCallback =
            Some(notification_callback_c);
        let delete_callback: UA_Client_DeleteMonitoredItemCallback = Some(delete_callback_c);
        let context = Context(St::prepare(st_tx));

        notification_callbacks.push(notification_callback);
        delete_callbacks.push(delete_callback);
        contexts.push(context);
        st_rxs.push(st_rx);
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
            UA_Client_MonitoredItems_createDataChanges_async(
                // SAFETY: Cast to `mut` pointer, function is marked `UA_THREADSAFE`.
                client.as_ptr().cast_mut(),
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
        .map(|response| (response, st_rxs))
}

fn delete_monitored_items(client: &ua::Client, request: &ua::DeleteMonitoredItemsRequest) {
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
