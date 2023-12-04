use std::{
    ffi::c_void,
    ptr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
};

use log::debug;
use open62541_sys::{
    UA_Client, UA_Client_readValueAttribute_async, UA_Client_run_iterate, UA_DataValue,
    UA_StatusCode, UA_UInt32, UA_STATUSCODE_GOOD,
};
use tokio::sync::oneshot;

use crate::{ua, Error};

pub struct AsyncClient {
    client: Arc<Mutex<ua::Client>>,
    dropped: Arc<AtomicBool>,
}

impl AsyncClient {
    pub(crate) fn from_sync(client: ua::Client) -> Self {
        let client = Arc::new(Mutex::new(client));
        let dropped = Arc::new(AtomicBool::new(false));

        {
            let client = client.clone();
            let dropped = dropped.clone();

            thread::spawn(move || {
                while !dropped.load(Ordering::Acquire) {
                    debug!("Running iterate");

                    let result = {
                        let mut client = client.lock().unwrap();
                        unsafe { UA_Client_run_iterate(client.as_mut_ptr(), 500) }
                    };
                    if result != UA_STATUSCODE_GOOD {
                        break;
                    }
                }
                debug!("Finished iterate loop");
            });
        }

        Self { client, dropped }
    }

    pub async fn read_value(&self, node_id: ua::NodeId) -> Result<ua::DataValue, Error> {
        type Tx = oneshot::Sender<Result<ua::DataValue, Error>>;
        let (tx, rx): (Tx, _) = oneshot::channel();

        unsafe extern "C" fn callback(
            _client: *mut UA_Client,
            userdata: *mut c_void,
            _request_id: UA_UInt32,
            status: UA_StatusCode,
            value: *mut UA_DataValue,
        ) {
            debug!("Processing read value response");
            let tx = Box::from_raw(userdata.cast::<Tx>());

            if status != UA_STATUSCODE_GOOD {
                // TODO: Do not panic here (FFI callback).
                tx.send(Err(Error::new(status))).unwrap();
                return;
            }

            let value = ua::DataValue::from_ref(&*value);
            // TODO: Do not panic here (FFI callback).
            tx.send(Ok(value)).unwrap();
        }

        let result = {
            debug!("Reading value attribute of {node_id}");
            let mut client = self.client.lock().unwrap();
            let tx = Box::into_raw(Box::new(tx));

            unsafe {
                UA_Client_readValueAttribute_async(
                    client.as_mut_ptr(),
                    node_id.into_inner(),
                    Some(callback),
                    tx.cast::<c_void>(),
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
    }
}

pub(crate) struct Package<T>(Box<T>);

impl<T> Package<T> {
    pub fn send(value: T) -> *mut c_void {
        let ptr: *mut T = Box::into_raw(Box::new(value));
        ptr.cast::<c_void>()
    }

    pub fn recv(raw: *mut c_void) -> T {
        let ptr: *mut T = raw.cast::<T>();
        *unsafe { Box::from_raw(ptr) }
    }
}
