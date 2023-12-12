use std::{
    ffi::c_void,
    ptr,
    sync::{Arc, Mutex},
    time::Duration,
};

use futures::Stream;
use log::debug;
use open62541_sys::{
    UA_Client, UA_Client_disconnect, UA_Client_readValueAttribute_async, UA_Client_run_iterate,
    UA_DataValue, UA_StatusCode, UA_UInt32, UA_STATUSCODE_GOOD,
};
use tokio::{sync::oneshot, task::JoinHandle, time};

use crate::{ua, AsyncSubscription, CallbackOnce, ClientBuilder, Error};

/// Connected OPC UA client (with asynchronous API).
pub struct AsyncClient {
    client: Arc<Mutex<ua::Client>>,
    background_handle: JoinHandle<()>,
    default_subscription: Arc<Mutex<Option<Arc<AsyncSubscription>>>>,
}

impl AsyncClient {
    /// Creates client connected to endpoint.
    ///
    /// If you need more control over the initialization, use [`ClientBuilder`] instead, and turn it
    /// into [`Client`](crate::Client) by calling [`connect()`](ClientBuilder::connect), followed by
    /// [`into_async()`](crate::Client::into_async) to get the asynchronous API.
    ///
    /// # Errors
    ///
    /// See [`ClientBuilder::connect()`].
    ///
    /// # Panics
    ///
    /// See [`ClientBuilder::connect()`].
    pub fn new(endpoint_url: &str) -> Result<Self, Error> {
        Ok(ClientBuilder::default().connect(endpoint_url)?.into_async())
    }

    pub(crate) fn from_sync(client: ua::Client) -> Self {
        let client = Arc::new(Mutex::new(client));

        let background_handle = {
            let client = Arc::clone(&client);

            // Run the event loop concurrently (this may be a different thread when using tokio with
            // `rt-multi-thread`). `UA_Client_run_iterate()` must be run periodically and makes sure
            // to maintain the connection (e.g. renew session) and run callback handlers.
            tokio::spawn(async move {
                loop {
                    debug!("Running iterate");

                    let result = {
                        let Ok(mut client) = client.lock() else {
                            break;
                        };

                        // Timeout of 0 means we do not block here at all. We don't want to hold the
                        // mutex longer than necessary (because that would block requests from being
                        // sent out).
                        unsafe { UA_Client_run_iterate(client.as_mut_ptr(), 0) }
                    };
                    if result != UA_STATUSCODE_GOOD {
                        break;
                    }

                    // This await point is where `background_handle.abort()` might abort us later.
                    time::sleep(Duration::from_millis(100)).await;
                }
            })
        };

        Self {
            client,
            background_handle,
            default_subscription: Arc::default(),
        }
    }

    /// Reads value from server.
    ///
    /// # Errors
    ///
    /// This fails when the node does not exist or its value attribute cannot be read.
    pub async fn read_value(&self, node_id: ua::NodeId) -> Result<ua::DataValue, Error> {
        read_value(&self.client, node_id).await
    }

    /// Creates new [subscription](AsyncSubscription).
    ///
    /// # Errors
    ///
    /// This fails when the client is not connected.
    pub async fn create_subscription(&self) -> Result<AsyncSubscription, Error> {
        AsyncSubscription::new(Arc::clone(&self.client)).await
    }

    /// Watches value for changes.
    ///
    /// This uses the internal default subscription to the server and adds a monitored item to it to
    /// subscribe the node for changes to its value attribute.
    ///
    /// # Errors
    ///
    /// This fails when the monitored item cannot be created. It also fails when (on the first call)
    /// the internal default subscription cannot be created.
    // TODO: Use async-aware lock.
    #[allow(clippy::await_holding_lock)]
    pub async fn value_stream(
        &self,
        node_id: ua::NodeId,
    ) -> Result<impl Stream<Item = ua::DataValue>, Error> {
        let subscription = {
            let Ok(mut default_subscription) = self.default_subscription.lock() else {
                return Err(Error::internal("should be able to lock subscription"));
            };

            if let Some(subscription) = default_subscription.as_ref() {
                // Use existing default subscription.
                Arc::clone(subscription)
            } else {
                // Create new subscription and store it for future monitored items.
                let subscription = Arc::new(self.create_subscription().await?);
                *default_subscription = Some(Arc::clone(&subscription));
                subscription
            }
        };

        let monitored_item = subscription.create_monitored_item(node_id).await?;

        Ok(monitored_item.into_stream())
    }
}

impl Drop for AsyncClient {
    fn drop(&mut self) {
        self.background_handle.abort();

        if let Ok(mut client) = self.client.lock() {
            let _unused = unsafe { UA_Client_disconnect(client.as_mut_ptr()) };
        }
    }
}

async fn read_value(
    client: &Arc<Mutex<ua::Client>>,
    node_id: ua::NodeId,
) -> Result<ua::DataValue, Error> {
    type Cb = CallbackOnce<Result<ua::DataValue, UA_StatusCode>>;

    unsafe extern "C" fn callback_c(
        _client: *mut UA_Client,
        userdata: *mut c_void,
        _request_id: UA_UInt32,
        status: UA_StatusCode,
        value: *mut UA_DataValue,
    ) {
        debug!("readValueAttribute() completed");

        let result = if status == UA_STATUSCODE_GOOD {
            // PANIC: We expect pointer to be valid when good.
            let value = value.as_ref().expect("value is set");
            Ok(ua::DataValue::from_ref(value))
        } else {
            Err(status)
        };
        Cb::execute(userdata, result);
    }

    let (tx, rx) = oneshot::channel::<Result<ua::DataValue, Error>>();

    let callback = |result: Result<ua::DataValue, _>| {
        // We always send a result back via `tx` (in fact, `rx.await` below expects this). We do not
        // care if that succeeds though: the receiver might already have gone out of scope (when its
        // future has been canceled) and we must not panic in FFI callbacks.
        let _unused = tx.send(result.map_err(Error::new));
    };

    let result = {
        let Ok(mut client) = client.lock() else {
            return Err(Error::internal("should be able to lock client"));
        };

        debug!("Calling readValueAttribute(), node_id={node_id:?}");

        unsafe {
            UA_Client_readValueAttribute_async(
                client.as_mut_ptr(),
                node_id.into_inner(),
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
    rx.await
        .unwrap_or(Err(Error::Internal("callback should send result")))
}
