use std::{
    ffi::c_void,
    ptr,
    sync::{Arc, Weak},
    time::Duration,
};

use futures_channel::oneshot;
use open62541_sys::{
    UA_Client, UA_Client_Subscriptions_create_async, UA_Client_Subscriptions_delete_async,
    UA_CreateSubscriptionResponse, UA_DeleteSubscriptionsResponse, UA_UInt32,
};

use crate::{
    ua, AsyncMonitoredItem, CallbackOnce, CreateMonitoredItemOptions, DataType as _, Error, Result,
};

#[derive(Debug, Default)]
pub struct CreateSubscriptionOptions {
    requested_publishing_interval: Option<Duration>,
    requested_lifetime_count: Option<u32>,
    requested_max_keep_alive_count: Option<u32>,
    max_notifications_per_publish: Option<u32>,
    publishing_enabled: Option<bool>,
    priority: Option<u8>,
}

impl CreateSubscriptionOptions {
    fn into_request(self) -> ua::CreateSubscriptionRequest {
        let Self {
            requested_publishing_interval,
            requested_lifetime_count,
            requested_max_keep_alive_count,
            max_notifications_per_publish,
            publishing_enabled,
            priority,
        } = self;

        let mut request = ua::CreateSubscriptionRequest::default();

        if let Some(requested_publishing_interval) = requested_publishing_interval {
            request = request.with_requested_publishing_interval(requested_publishing_interval);
        }
        if let Some(requested_lifetime_count) = requested_lifetime_count {
            request = request.with_requested_lifetime_count(requested_lifetime_count);
        }
        if let Some(requested_max_keep_alive_count) = requested_max_keep_alive_count {
            request = request.with_requested_max_keep_alive_count(requested_max_keep_alive_count);
        }
        if let Some(max_notifications_per_publish) = max_notifications_per_publish {
            request = request.with_max_notifications_per_publish(max_notifications_per_publish);
        }
        if let Some(publishing_enabled) = publishing_enabled {
            request = request.with_publishing_enabled(publishing_enabled);
        }
        if let Some(priority) = priority {
            request = request.with_priority(priority);
        }

        request
    }
}

/// Subscription (with asynchronous API).
#[derive(Debug)]
pub struct AsyncSubscription {
    client: Weak<ua::Client>,
    subscription_id: ua::SubscriptionId,
}

impl AsyncSubscription {
    pub(crate) async fn new(
        client: &Arc<ua::Client>,
        options: CreateSubscriptionOptions,
    ) -> Result<Self> {
        let request = options.into_request();

        let response = create_subscription(client, &request).await?;

        Ok(AsyncSubscription {
            client: Arc::downgrade(client),
            subscription_id: response.subscription_id(),
        })
    }

    /// Creates [monitored item](AsyncMonitoredItem).
    ///
    /// This creates a new monitored item for the given node.
    ///
    /// # Errors
    ///
    /// This fails when the node does not exist.
    pub async fn create_monitored_item(&self, node_id: &ua::NodeId) -> Result<AsyncMonitoredItem> {
        self.create_monitored_item_with_options(node_id, CreateMonitoredItemOptions::default())
            .await
    }

    /// Creates [monitored item](AsyncMonitoredItem) with options.
    ///
    /// This creates a new monitored item for the given node, it allows overriding the parameters
    /// used in the request.
    ///
    /// # Errors
    ///
    /// This fails when the node does not exist.
    pub async fn create_monitored_item_with_options(
        &self,
        node_id: &ua::NodeId,
        options: CreateMonitoredItemOptions,
    ) -> Result<AsyncMonitoredItem> {
        let Some(client) = self.client.upgrade() else {
            return Err(Error::internal("client should not be dropped"));
        };

        AsyncMonitoredItem::new(&client, self.subscription_id, node_id, options).await
    }
}

impl Drop for AsyncSubscription {
    fn drop(&mut self) {
        let Some(client) = self.client.upgrade() else {
            return;
        };

        let request =
            ua::DeleteSubscriptionsRequest::init().with_subscription_ids(&[self.subscription_id]);

        delete_subscriptions(&client, &request);
    }
}

async fn create_subscription(
    client: &ua::Client,
    request: &ua::CreateSubscriptionRequest,
) -> Result<ua::CreateSubscriptionResponse> {
    type Cb = CallbackOnce<std::result::Result<ua::CreateSubscriptionResponse, ua::StatusCode>>;

    unsafe extern "C" fn callback_c(
        _client: *mut UA_Client,
        userdata: *mut c_void,
        _request_id: UA_UInt32,
        response: *mut c_void,
    ) {
        log::debug!("Subscriptions_create() completed");

        let response = response.cast::<UA_CreateSubscriptionResponse>();
        // SAFETY: Incoming pointer is valid for access.
        // PANIC: We expect pointer to be valid when good.
        let response = unsafe { response.as_ref() }.expect("response should be set");
        let status_code = ua::StatusCode::new(response.responseHeader.serviceResult);

        let result = if status_code.is_good() {
            Ok(ua::CreateSubscriptionResponse::clone_raw(response))
        } else {
            Err(status_code)
        };

        // SAFETY: `userdata` is the result of `Cb::prepare()` and is used only once.
        unsafe {
            Cb::execute(userdata, result);
        }
    }

    let (tx, rx) = oneshot::channel::<Result<ua::CreateSubscriptionResponse>>();

    let callback = |result: std::result::Result<ua::CreateSubscriptionResponse, _>| {
        // We always send a result back via `tx` (in fact, `rx.await` below expects this). We do not
        // care if that succeeds though: the receiver might already have gone out of scope (when its
        // future has been cancelled) and we must not panic in FFI callbacks.
        let _unused = tx.send(result.map_err(Error::new));
    };

    let status_code = ua::StatusCode::new({
        log::debug!("Calling Subscriptions_create()");

        // SAFETY: `UA_Client_Subscriptions_create_async()` expects the request passed by value but
        // does not take ownership.
        let request = unsafe { ua::CreateSubscriptionRequest::to_raw_copy(request) };

        unsafe {
            UA_Client_Subscriptions_create_async(
                // SAFETY: Cast to `mut` pointer, function is marked `UA_THREADSAFE`.
                client.as_ptr().cast_mut(),
                request,
                ptr::null_mut(),
                None,
                None,
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
}

fn delete_subscriptions(client: &ua::Client, request: &ua::DeleteSubscriptionsRequest) {
    unsafe extern "C" fn callback_c(
        _client: *mut UA_Client,
        _userdata: *mut c_void,
        _request_id: UA_UInt32,
        response: *mut c_void,
    ) {
        log::debug!("Subscriptions_delete() completed");

        let response = response.cast::<UA_DeleteSubscriptionsResponse>();
        // SAFETY: Incoming pointer is valid for access.
        // PANIC: We expect pointer to be valid when good.
        let response = unsafe { response.as_ref() }.expect("response should be set");
        let status_code = ua::StatusCode::new(response.responseHeader.serviceResult);

        if let Err(error) = Error::verify_good(&status_code) {
            log::warn!("Error in response when deleting subscriptions: {error}");
        }
    }

    let status_code = ua::StatusCode::new({
        log::debug!("Calling Subscriptions_delete()");

        // SAFETY: `UA_Client_Subscriptions_delete_async()` expects the request passed by value but
        // does not take ownership.
        let request = unsafe { ua::DeleteSubscriptionsRequest::to_raw_copy(request) };

        unsafe {
            UA_Client_Subscriptions_delete_async(
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
        log::warn!("Error in request when deleting subscriptions: {error}");
    }
}
