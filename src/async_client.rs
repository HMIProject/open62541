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

use crate::{ua, AsyncSubscription, CallbackOnce, Error};

pub struct AsyncClient {
    client: Arc<Mutex<ua::Client>>,
    background_handle: JoinHandle<()>,
    default_subscription: Arc<Mutex<Option<Arc<AsyncSubscription>>>>,
}

impl AsyncClient {
    pub(crate) fn from_sync(client: ua::Client) -> Self {
        let client = Arc::new(Mutex::new(client));

        let background_handle = {
            let client = client.clone();

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
            default_subscription: Default::default(),
        }
    }

    pub async fn read_value(&self, node_id: ua::NodeId) -> Result<ua::DataValue, Error> {
        type Cb = CallbackOnce<Result<ua::DataValue, UA_StatusCode>>;

        unsafe extern "C" fn callback_c(
            _client: *mut UA_Client,
            userdata: *mut c_void,
            _request_id: UA_UInt32,
            status: UA_StatusCode,
            value: *mut UA_DataValue,
        ) {
            let result = if status == UA_STATUSCODE_GOOD {
                Ok(ua::DataValue::from_ref(&*value))
            } else {
                Err(status)
            };
            Cb::execute(userdata, result);
        }

        let (tx, rx) = oneshot::channel::<Result<ua::DataValue, Error>>();

        let callback = |result: Result<_, _>| {
            debug!("Processing read value response");

            // We always send a result back via `tx` (in fact, `rx.await` below expects this). We do
            // not care if that succeeds, however: the receiver might already have gone out of scope
            // (when its future has been canceled) and we must not panic in FFI callbacks.
            let _unused = tx.send(result.map_err(|status| Error::new(status)));
        };

        let result = {
            debug!("Reading value attribute of {node_id}");
            let mut client = self.client.lock().unwrap();

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

        // PANIC: When `callback` is called (which owns `tx`), we always call `tx.send()`. Thus, the
        // sender is only dropped after placing a value into the channel and `rx.await` always finds
        // this value there.
        rx.await.expect("callback always sends value")
    }

    pub async fn create_subscription(&self) -> Result<AsyncSubscription, Error> {
        AsyncSubscription::new(self.client.clone()).await
    }

    pub async fn watch_value(
        &self,
        node_id: ua::NodeId,
    ) -> Result<impl Stream<Item = ua::DataValue>, Error> {
        let subscription = {
            let mut default_subscription = self.default_subscription.lock().unwrap();

            match &*default_subscription {
                Some(subscription) => subscription.clone(),
                None => {
                    let subscription = Arc::new(self.create_subscription().await?);
                    *default_subscription = Some(subscription.clone());
                    subscription
                }
            }
        };

        let monitored_item = subscription.monitor_item(node_id).await?;

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
