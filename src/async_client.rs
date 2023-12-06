use std::{
    ffi::c_void,
    ptr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use log::debug;
use open62541_sys::{
    UA_Client, UA_Client_disconnect, UA_Client_readValueAttribute_async, UA_Client_run_iterate,
    UA_DataValue, UA_StatusCode, UA_UInt32, UA_STATUSCODE_GOOD,
};
use tokio::sync::oneshot;

use crate::{ua, AsyncSubscription, CallbackOnce, Error};

pub struct AsyncClient {
    client: Arc<Mutex<ua::Client>>,
    dropped: Arc<AtomicBool>,
    loop_handle: JoinHandle<()>,
}

impl AsyncClient {
    pub(crate) fn from_sync(client: ua::Client) -> Self {
        let client = Arc::new(Mutex::new(client));
        let dropped = Arc::new(AtomicBool::new(false));

        let loop_handle = {
            let client = client.clone();
            let dropped = dropped.clone();

            // Run the event loop in a different thread. For callbacks, `UA_Client_run_iterate()` is
            // to be run periodically in the background and that is what we do here.
            thread::spawn(move || {
                while !dropped.load(Ordering::Acquire) {
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

                    thread::park_timeout(Duration::from_millis(100));
                }

                debug!("Finished iterate loop");
            })
        };

        Self {
            client,
            dropped,
            loop_handle,
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
}

impl Drop for AsyncClient {
    fn drop(&mut self) {
        self.dropped.store(true, Ordering::Release);
        self.loop_handle.thread().unpark();

        if let Ok(mut client) = self.client.lock() {
            let _unused = unsafe { UA_Client_disconnect(client.as_mut_ptr()) };
        }
    }
}
