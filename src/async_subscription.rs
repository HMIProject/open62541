use std::{
    ffi::c_void,
    ptr,
    sync::{Arc, Mutex, Weak},
};

use futures::channel::oneshot;
use log::debug;
use open62541_sys::{
    UA_Client, UA_Client_Subscriptions_create_async, UA_Client_Subscriptions_delete_async,
    UA_CreateSubscriptionResponse, UA_StatusCode, UA_UInt32, UA_STATUSCODE_GOOD,
};

use crate::{callback::CallbackOnce, ua, Error, SubscriptionId};

pub struct AsyncSubscription {
    client: Weak<Mutex<ua::Client>>,
    subscription_id: SubscriptionId,
}

impl AsyncSubscription {
    pub(crate) async fn new(client: Arc<Mutex<ua::Client>>) -> Result<Self, Error> {
        type Cb = CallbackOnce<Result<ua::CreateSubscriptionResponse, UA_StatusCode>>;

        unsafe extern "C" fn callback_c(
            _client: *mut UA_Client,
            userdata: *mut c_void,
            _request_id: UA_UInt32,
            response: *mut c_void,
        ) {
            debug!("Subscriptions_create() completed");

            let response = response.cast::<UA_CreateSubscriptionResponse>();
            let status = (*response).responseHeader.serviceResult;
            let result = if status == UA_STATUSCODE_GOOD {
                Ok(ua::CreateSubscriptionResponse::from_ref(&*response))
            } else {
                Err(status)
            };
            Cb::execute(userdata, result);
        }

        let (tx, rx) = oneshot::channel::<Result<AsyncSubscription, Error>>();
        let weak_client = Arc::downgrade(&client);

        let callback = |result: Result<ua::CreateSubscriptionResponse, _>| {
            // We always send a result back via `tx` (in fact, `rx.await` below expects this). We do
            // not care if that succeeds, however: the receiver might already have gone out of scope
            // (when its future has been canceled) and we must not panic in FFI callbacks.
            let _unused = tx.send(
                result
                    .map(|result| AsyncSubscription {
                        client: weak_client,
                        subscription_id: result.subscription_id(),
                    })
                    .map_err(Error::new),
            );
        };

        let request = ua::CreateSubscriptionRequest::default();

        let result = {
            let mut client = client.lock().unwrap();

            debug!("Calling Subscriptions_create()");

            unsafe {
                UA_Client_Subscriptions_create_async(
                    client.as_mut_ptr(),
                    request.into_inner(),
                    ptr::null_mut(),
                    None,
                    None,
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
        rx.await.unwrap()
    }
}

impl Drop for AsyncSubscription {
    fn drop(&mut self) {
        unsafe extern "C" fn callback_c(
            _client: *mut UA_Client,
            _userdata: *mut c_void,
            _request_id: UA_UInt32,
            _response: *mut c_void,
        ) {
            debug!("Subscriptions_delete() completed");

            // Nothing to do here.
        }

        let Some(client) = self.client.upgrade() else {
            return;
        };

        let request =
            ua::DeleteSubscriptionsRequest::init().with_subscription_ids(&[self.subscription_id]);

        let _unused = {
            let Ok(mut client) = client.lock() else {
                return;
            };

            debug!("Calling Subscriptions_delete()");

            unsafe {
                UA_Client_Subscriptions_delete_async(
                    client.as_mut_ptr(),
                    request.into_inner(),
                    // Required. Handler installed by `UA_Client_Subscriptions_delete_async()` calls
                    // it unconditionally (as opposed to other callbacks which may be left unset).
                    Some(callback_c),
                    ptr::null_mut(),
                    ptr::null_mut(),
                )
            }
        };
    }
}
