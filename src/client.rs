use std::{
    ffi::{c_void, CString},
    ptr,
};

use log::info;
use open62541_sys::{
    UA_AttributeId_UA_ATTRIBUTEID_NODEID, UA_AttributeId_UA_ATTRIBUTEID_VALUE,
    UA_ClientConfig_setDefault, UA_Client_MonitoredItems_createDataChange,
    UA_Client_MonitoredItems_createDataChanges, UA_Client_Service_read,
    UA_Client_Subscriptions_create, UA_Client_Subscriptions_delete, UA_Client_connect,
    UA_Client_getConfig, UA_Client_run_iterate, UA_TimestampsToReturn_UA_TIMESTAMPSTORETURN_BOTH,
    __UA_Client_readAttribute, UA_STATUSCODE_GOOD, UA_TYPES, UA_TYPES_NODEID, UA_TYPES_VARIANT,
};

#[cfg(feature = "tokio")]
use crate::AsyncClient;
use crate::{ua, DataType, Error, SubscriptionId};

/// Builder for [`Client`].
///
/// Use this to specify additional options before connecting to an OPC UA endpoint.
#[allow(clippy::module_name_repetitions)]
pub struct ClientBuilder(ua::Client);

impl ClientBuilder {
    /// Connects to OPC UA endpoint and returns [`Client`].
    ///
    /// # Errors
    ///
    /// This fails when the target server is not reachable.
    ///
    /// # Panics
    ///
    /// The endpoint URL must be a valid C string, i.e. it must not contain any NUL bytes.
    pub fn connect(mut self, endpoint_url: &str) -> Result<Client, Error> {
        info!("Connecting to endpoint {endpoint_url}");

        let endpoint_url =
            CString::new(endpoint_url).expect("endpoint URL does not contain NUL bytes");

        let result = unsafe { UA_Client_connect(self.0.as_mut_ptr(), endpoint_url.as_ptr()) };
        if result != UA_STATUSCODE_GOOD {
            return Err(Error::new(result));
        }

        Ok(Client(self.0))
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        let mut inner = ua::Client::default();

        // Clients need to be initialized with config for `UA_Client_connect` to work.
        let result = unsafe {
            let config = UA_Client_getConfig(inner.as_mut_ptr());
            UA_ClientConfig_setDefault(config)
        };
        assert!(result == UA_STATUSCODE_GOOD);

        Self(inner)
    }
}

/// Connected OPC UA client.
///
/// This represents an OPC UA client connected to a specific endpoint. Once a client is connected to
/// an endpoint, it is not possible to switch to another server. Create a new client for that.
///
/// Once a connection to the given endpoint is established, the client keeps the connection open and
/// reconnects if necessary.
///
/// If the connection fails unrecoverably, the client is no longer usable. In this case create a new
/// client if required.
pub struct Client(ua::Client);

impl Client {
    /// Creates client connected to endpoint.
    ///
    /// If you need more control over the initialization, use [`ClientBuilder`] instead, and turn it
    /// into [`Client`] by calling [`connect()`](ClientBuilder::connect()).
    ///
    /// # Errors
    ///
    /// See [`ClientBuilder::connect()`].
    ///
    /// # Panics
    ///
    /// See [`ClientBuilder::connect()`].
    pub fn new(endpoint_url: &str) -> Result<Self, Error> {
        ClientBuilder::default().connect(endpoint_url)
    }

    /// Turns client into [`AsyncClient`].
    ///
    /// The [`AsyncClient`] can be used to access methods in an asynchronous way.
    #[must_use]
    #[cfg(feature = "tokio")]
    pub fn into_async(self) -> AsyncClient {
        AsyncClient::from_sync(self.0)
    }

    /// Run event loop iteration.
    ///
    /// This should be called periodically to process background events and trigger callbacks when a
    /// message arrives asynchronously.
    ///
    /// # Errors
    ///
    /// This fails when the request cannot be served.
    pub fn run_iterate(&mut self) -> Result<(), Error> {
        // TODO: Allow setting this.
        let timeout = 500;

        let result = unsafe { UA_Client_run_iterate(self.0.as_mut_ptr(), timeout) };
        if result != UA_STATUSCODE_GOOD {
            return Err(Error::new(result));
        }

        Ok(())
    }

    /// Read data from server.
    ///
    /// # Errors
    ///
    /// This fails when the request cannot be served.
    pub fn read(&mut self, request: ua::ReadRequest) -> Result<ua::ReadResponse, Error> {
        let response = unsafe { UA_Client_Service_read(self.0.as_mut_ptr(), request.into_inner()) };
        if response.responseHeader.serviceResult != UA_STATUSCODE_GOOD {
            return Err(Error::new(response.responseHeader.serviceResult));
        }

        Ok(ua::ReadResponse::new(response))
    }

    /// Read node ID attribute from node.
    ///
    /// # Errors
    ///
    /// This fails when the node does not exist or the node ID attribute cannot be read.
    pub fn read_node_id(&mut self, node_id: &ua::NodeId) -> Result<ua::NodeId, Error> {
        let mut output = ua::NodeId::init();
        let data_type = unsafe { &UA_TYPES[UA_TYPES_NODEID as usize] };

        let result = unsafe {
            __UA_Client_readAttribute(
                self.0.as_mut_ptr(),
                node_id.as_ptr(),
                UA_AttributeId_UA_ATTRIBUTEID_NODEID,
                output.as_mut_ptr().cast::<c_void>(),
                data_type,
            )
        };
        if result != UA_STATUSCODE_GOOD {
            return Err(Error::new(result));
        }

        Ok(output)
    }

    /// Read value attribute from node.
    ///
    /// # Errors
    ///
    /// This fails when the node does not exist or the value attribute cannot be read.
    pub fn read_value(&mut self, node_id: &ua::NodeId) -> Result<ua::Variant, Error> {
        let mut output = ua::Variant::init();
        let data_type = unsafe { &UA_TYPES[UA_TYPES_VARIANT as usize] };

        let result = unsafe {
            __UA_Client_readAttribute(
                self.0.as_mut_ptr(),
                node_id.as_ptr(),
                UA_AttributeId_UA_ATTRIBUTEID_VALUE,
                output.as_mut_ptr().cast::<c_void>(),
                data_type,
            )
        };
        if result != UA_STATUSCODE_GOOD {
            return Err(Error::new(result));
        }

        Ok(output)
    }

    /// Create subscription.
    ///
    /// # Errors
    ///
    /// This fails when the request cannot be served.
    pub fn create_subscription(
        &mut self,
        request: ua::CreateSubscriptionRequest,
    ) -> Result<ua::CreateSubscriptionResponse, Error> {
        let response = unsafe {
            UA_Client_Subscriptions_create(
                self.0.as_mut_ptr(),
                request.into_inner(),
                ptr::null_mut(),
                None,
                None,
            )
        };
        if response.responseHeader.serviceResult != UA_STATUSCODE_GOOD {
            return Err(Error::new(response.responseHeader.serviceResult));
        }

        Ok(ua::CreateSubscriptionResponse::new(response))
    }

    /// Delete subscriptions.
    ///
    /// # Errors
    ///
    /// This fails when the request cannot be served.
    pub fn delete_subscriptions(
        &mut self,
        request: ua::DeleteSubscriptionsRequest,
    ) -> Result<ua::DeleteSubscriptionsResponse, Error> {
        let response =
            unsafe { UA_Client_Subscriptions_delete(self.0.as_mut_ptr(), request.into_inner()) };
        if response.responseHeader.serviceResult != UA_STATUSCODE_GOOD {
            return Err(Error::new(response.responseHeader.serviceResult));
        }

        Ok(ua::DeleteSubscriptionsResponse::new(response))
    }

    pub fn create_data_changes(
        &mut self,
        request: ua::CreateMonitoredItemsRequest,
    ) -> Result<ua::CreateMonitoredItemsResponse, Error> {
        // TODO: Implement this.
        let contexts = todo!();
        let callbacks = todo!();
        let delete_callbacks = todo!();

        let response = unsafe {
            UA_Client_MonitoredItems_createDataChanges(
                self.0.as_mut_ptr(),
                request.into_inner(),
                contexts,
                callbacks,
                delete_callbacks,
            )
        };
        if response.responseHeader.serviceResult != UA_STATUSCODE_GOOD {
            return Err(Error::new(response.responseHeader.serviceResult));
        }

        Ok(ua::CreateMonitoredItemsResponse::new(response))
    }

    /// Watch monitored item for data change.
    ///
    /// # Errors
    ///
    /// This fails when the request cannot be served.
    pub fn create_data_change<F: Fn(ua::DataValue) + 'static>(
        &mut self,
        subscription_id: SubscriptionId,
        item: ua::MonitoredItemCreateRequest,
        callback: F,
    ) -> Result<ua::MonitoredItemCreateResult, Error> {
        // TODO: Allow setting this.
        let timestamps_to_return = UA_TimestampsToReturn_UA_TIMESTAMPSTORETURN_BOTH;

        let callback = CallbackFn(Box::new(callback));
        let callback_box = Box::new(callback);
        let callback_ptr = Box::into_raw(callback_box);

        let result = unsafe {
            UA_Client_MonitoredItems_createDataChange(
                self.0.as_mut_ptr(),
                subscription_id.0,
                timestamps_to_return,
                item.into_inner(),
                callback_ptr.cast::<c_void>(),
                Some(extern_callback),
                Some(extern_delete),
            )
        };
        if result.statusCode != UA_STATUSCODE_GOOD {
            return Err(Error::new(result.statusCode));
        }

        Ok(ua::MonitoredItemCreateResult::new(result))
    }
}

struct CallbackFn(Box<dyn Fn(ua::DataValue)>);

extern "C" fn extern_callback(
    _client: *mut open62541_sys::UA_Client,
    _sub_id: open62541_sys::UA_UInt32,
    _sub_context: *mut c_void,
    _mon_id: open62541_sys::UA_UInt32,
    mon_context: *mut c_void,
    value: *mut open62541_sys::UA_DataValue,
) {
    let callback_ptr = mon_context.cast::<CallbackFn>();
    let callback_box = unsafe { Box::from_raw(callback_ptr) };

    if let Some(value) = unsafe { value.as_ref() } {
        callback_box.0(ua::DataValue::from_ref(value));
    }

    let callback_ptr = Box::into_raw(callback_box);
    debug_assert_eq!(callback_ptr.cast::<c_void>(), mon_context);
}

extern "C" fn extern_delete(
    _client: *mut open62541_sys::UA_Client,
    _sub_id: open62541_sys::UA_UInt32,
    _sub_context: *mut c_void,
    _mon_id: open62541_sys::UA_UInt32,
    mon_context: *mut c_void,
) {
    let callback_ptr = mon_context.cast::<CallbackFn>();
    let callback_box = unsafe { Box::from_raw(callback_ptr) };

    drop(callback_box);
}
