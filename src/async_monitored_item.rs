use std::{
    ffi::c_void,
    ptr,
    sync::{Arc, Mutex, Weak},
};

use futures::channel::oneshot;
use log::debug;
use open62541_sys::{
    UA_Client, UA_Client_DataChangeNotificationCallback, UA_Client_DeleteMonitoredItemCallback,
    UA_Client_MonitoredItems_createDataChanges_async, UA_Client_MonitoredItems_delete_async,
    UA_CreateMonitoredItemsResponse, UA_DataValue, UA_StatusCode, UA_UInt32, UA_STATUSCODE_GOOD,
};

use crate::{callback::CallbackOnce, ua, Error, MonitoredItemId, SubscriptionId};

pub struct AsyncMonitoredItem {
    client: Weak<Mutex<ua::Client>>,
    monitored_item_id: MonitoredItemId,
}

impl AsyncMonitoredItem {
    pub(crate) async fn new(
        client: Arc<Mutex<ua::Client>>,
        subscription_id: SubscriptionId,
        node_id: ua::NodeId,
    ) -> Result<Self, Error> {
        let request = ua::CreateMonitoredItemsRequest::init()
            .with_subscription_id(subscription_id)
            .with_items_to_create(&[ua::MonitoredItemCreateRequest::init_node_id(node_id)]);

        let response = create_monitored_items(client.clone(), request).await?;

        // PANIC: We expect exactly one result for the monitored item we requested above.
        let monitored_item_id = *response.monitored_item_ids().unwrap().get(0).unwrap();

        Ok(AsyncMonitoredItem {
            client: Arc::downgrade(&client),
            monitored_item_id,
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

        delete_monitored_items(client, request);
    }
}

async fn create_monitored_items(
    client: Arc<Mutex<ua::Client>>,
    request: ua::CreateMonitoredItemsRequest,
) -> Result<ua::CreateMonitoredItemsResponse, Error> {
    type Cb = CallbackOnce<Result<ua::CreateMonitoredItemsResponse, UA_StatusCode>>;

    unsafe extern "C" fn notification_callback_c(
        _client: *mut UA_Client,
        _sub_id: UA_UInt32,
        _sub_context: *mut ::std::os::raw::c_void,
        _mon_id: UA_UInt32,
        _mon_context: *mut ::std::os::raw::c_void,
        _value: *mut UA_DataValue,
    ) {
        debug!("DataChangeNotificationCallback() was called");

        // TODO
    }

    unsafe extern "C" fn delete_callback_c(
        _client: *mut UA_Client,
        _sub_id: UA_UInt32,
        _sub_context: *mut ::std::os::raw::c_void,
        _mon_id: UA_UInt32,
        _mon_context: *mut ::std::os::raw::c_void,
    ) {
        debug!("DeleteMonitoredItemCallback() was called");

        // TODO
    }

    unsafe extern "C" fn callback_c(
        _client: *mut UA_Client,
        userdata: *mut c_void,
        _request_id: UA_UInt32,
        response: *mut c_void,
    ) {
        debug!("MonitoredItems_createDataChanges() completed");

        let response = response.cast::<UA_CreateMonitoredItemsResponse>();
        let status = (*response).responseHeader.serviceResult;
        let result = if status == UA_STATUSCODE_GOOD {
            Ok(ua::CreateMonitoredItemsResponse::from_ref(&*response))
        } else {
            Err(status)
        };
        Cb::execute(userdata, result);
    }

    let (tx, rx) = oneshot::channel::<Result<ua::CreateMonitoredItemsResponse, Error>>();

    let callback = |result: Result<ua::CreateMonitoredItemsResponse, _>| {
        // We always send a result back via `tx` (in fact, `rx.await` below expects this). We do not
        // care if that succeeds though: the receiver might already have gone out of scope (when its
        // future has been canceled) and we must not panic in FFI callbacks.
        let _unused = tx.send(result.map_err(Error::new));
    };

    let mut notification_callbacks: Vec<UA_Client_DataChangeNotificationCallback> =
        vec![Some(notification_callback_c)];
    let mut delete_callbacks: Vec<UA_Client_DeleteMonitoredItemCallback> =
        vec![Some(delete_callback_c)];

    let result = {
        let mut client = client.lock().unwrap();

        debug!("Calling MonitoredItems_createDataChanges()");

        unsafe {
            UA_Client_MonitoredItems_createDataChanges_async(
                client.as_mut_ptr(),
                request.into_inner(),
                ptr::null_mut(),
                notification_callbacks.as_mut_ptr(),
                delete_callbacks.as_mut_ptr(),
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
    rx.await.unwrap()
}

fn delete_monitored_items(
    client: Arc<Mutex<ua::Client>>,
    request: ua::DeleteMonitoredItemsRequest,
) {
    unsafe extern "C" fn callback_c(
        _client: *mut UA_Client,
        _userdata: *mut c_void,
        _request_id: UA_UInt32,
        _response: *mut c_void,
    ) {
        debug!("MonitoredItems_delete() completed");

        // Nothing to do here.
    }

    let _unused = {
        let Ok(mut client) = client.lock() else {
            return;
        };

        debug!("Calling MonitoredItems_delete()");

        unsafe {
            UA_Client_MonitoredItems_delete_async(
                client.as_mut_ptr(),
                request.into_inner(),
                // This must be set (despite the `Option` type). The internal handler in `open62541`
                // calls our callback unconditionally (as opposed to other service functions where a
                // handler may be left unset if not required).
                Some(callback_c),
                ptr::null_mut(),
                ptr::null_mut(),
            )
        }
    };
}
