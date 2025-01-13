use std::{
    ffi::c_void,
    num::NonZeroU32,
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
    ua, AsyncClient, AsyncMonitoredItem, CallbackOnce, DataType as _, Error, MonitoredItemBuilder,
    Result,
};

#[derive(Debug, Default)]
pub struct SubscriptionBuilder {
    #[allow(clippy::option_option)]
    requested_publishing_interval: Option<Option<Duration>>,
    requested_lifetime_count: Option<u32>,
    #[allow(clippy::option_option)]
    requested_max_keep_alive_count: Option<Option<NonZeroU32>>,
    #[allow(clippy::option_option)]
    max_notifications_per_publish: Option<Option<NonZeroU32>>,
    publishing_enabled: Option<bool>,
    priority: Option<u8>,
}

// Note: The default values in the docs below come from `UA_CreateSubscriptionRequest_default()`.
impl SubscriptionBuilder {
    /// Sets requested publishing interval.
    ///
    /// Default value is 500.0 ms.
    ///
    /// See [`ua::CreateSubscriptionRequest::with_requested_publishing_interval()`].
    #[must_use]
    pub const fn requested_publishing_interval(
        mut self,
        requested_publishing_interval: Option<Duration>,
    ) -> Self {
        self.requested_publishing_interval = Some(requested_publishing_interval);
        self
    }

    /// Sets requested lifetime count.
    ///
    /// Default value is 10000.
    ///
    /// See [`ua::CreateSubscriptionRequest::with_requested_lifetime_count()`].
    #[must_use]
    pub const fn requested_lifetime_count(mut self, requested_lifetime_count: u32) -> Self {
        self.requested_lifetime_count = Some(requested_lifetime_count);
        self
    }

    /// Sets requested maximum keep-alive count.
    ///
    /// Default value is 10.
    ///
    /// See [`ua::CreateSubscriptionRequest::with_requested_max_keep_alive_count()`].
    #[must_use]
    pub const fn requested_max_keep_alive_count(
        mut self,
        requested_max_keep_alive_count: Option<NonZeroU32>,
    ) -> Self {
        self.requested_max_keep_alive_count = Some(requested_max_keep_alive_count);
        self
    }

    /// Sets maximum number of notifications that the client wishes to receive in a single publish
    /// response.
    ///
    /// Default value is `None` (unlimited).
    ///
    /// See [`ua::CreateSubscriptionRequest::with_max_notifications_per_publish()`].
    #[must_use]
    pub const fn max_notifications_per_publish(
        mut self,
        max_notifications_per_publish: Option<NonZeroU32>,
    ) -> Self {
        self.max_notifications_per_publish = Some(max_notifications_per_publish);
        self
    }

    /// Enables or disables publishing.
    ///
    /// Default value is `true`.
    ///
    /// See [`ua::CreateSubscriptionRequest::with_publishing_enabled()`].
    #[must_use]
    pub const fn publishing_enabled(mut self, publishing_enabled: bool) -> Self {
        self.publishing_enabled = Some(publishing_enabled);
        self
    }

    /// Sets relative priority of the subscription.
    ///
    /// Default value is 0.
    ///
    /// See [`ua::CreateSubscriptionRequest::with_priority()`].
    #[must_use]
    pub const fn priority(mut self, priority: u8) -> Self {
        self.priority = Some(priority);
        self
    }

    /// Creates subscription.
    ///
    /// # Errors
    ///
    /// This fails when the client is not connected.
    pub async fn create(
        self,
        client: &AsyncClient,
    ) -> Result<(ua::CreateSubscriptionResponse, AsyncSubscription)> {
        let client = client.client();

        let response = create_subscription(client, &self.into_request()).await?;

        let subscription = AsyncSubscription {
            client: Arc::downgrade(client),
            subscription_id: response.subscription_id(),
        };

        Ok((response, subscription))
    }

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
    /// Creates [monitored item](AsyncMonitoredItem).
    ///
    /// This creates a new monitored item for the given node.
    ///
    /// # Errors
    ///
    /// This fails when the node does not exist.
    pub async fn create_monitored_item(&self, node_id: &ua::NodeId) -> Result<AsyncMonitoredItem> {
        let results = MonitoredItemBuilder::new([node_id.clone()])
            .create(self)
            .await?;

        // We expect exactly one result for the single monitored item we requested above.
        let Ok::<[_; 1], _>([result]) = results.try_into() else {
            return Err(Error::internal("expected exactly one monitored item"));
        };

        // Verify single item's status code and return as error.
        let (_, monitored_item) = result?;

        Ok(monitored_item)
    }

    #[must_use]
    pub(crate) const fn client(&self) -> &Weak<ua::Client> {
        &self.client
    }

    #[must_use]
    pub(crate) const fn subscription_id(&self) -> ua::SubscriptionId {
        self.subscription_id
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
