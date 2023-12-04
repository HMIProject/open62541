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
    UA_Client, UA_Client_readValueAttribute_async, UA_Client_run_iterate, UA_DataValue,
    UA_StatusCode, UA_UInt32, UA_STATUSCODE_GOOD,
};
use tokio::sync::oneshot;

use crate::{ua, CallbackOnce, Error};

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

            thread::spawn(move || {
                while !dropped.load(Ordering::Acquire) {
                    debug!("Running iterate");

                    let result = {
                        let mut client = client.lock().unwrap();
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
        let (tx, rx) = oneshot::channel::<Result<ua::DataValue, Error>>();

        type Cb = CallbackOnce<(UA_StatusCode, ua::DataValue)>;

        unsafe extern "C" fn callback_c(
            _client: *mut UA_Client,
            userdata: *mut c_void,
            _request_id: UA_UInt32,
            status: UA_StatusCode,
            value: *mut UA_DataValue,
        ) {
            Cb::execute(userdata, (status, ua::DataValue::from_ref(&*value)));
        }

        let callback = move |(status, value)| {
            debug!("Processing read value response");

            if status != UA_STATUSCODE_GOOD {
                // TODO: Do not panic here (FFI callback).
                tx.send(Err(Error::new(status))).unwrap();
                return;
            }

            // TODO: Do not panic here (FFI callback).
            tx.send(Ok(value)).unwrap();
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

        // TODO: Handle channel error.
        rx.await.unwrap()
    }
}

impl Drop for AsyncClient {
    fn drop(&mut self) {
        self.dropped.store(true, Ordering::Release);
        self.loop_handle.thread().unpark();
    }
}
