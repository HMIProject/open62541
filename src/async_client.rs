use std::{
    ffi::c_void,
    ptr,
    sync::{Arc, Mutex},
    time::Duration,
};

use open62541_sys::{
    UA_Client, UA_Client_disconnect, UA_Client_readValueAttribute_async, UA_Client_run_iterate,
    UA_Client_sendAsyncRequest, UA_DataValue, UA_StatusCode, UA_UInt32, UA_STATUSCODE_GOOD,
};
use tokio::{sync::oneshot, task::JoinHandle, time};

use crate::{
    ua, AsyncSubscription, CallbackOnce, ClientBuilder, DataType, Error, ServiceRequest,
    ServiceResponse,
};

/// Connected OPC UA client (with asynchronous API).
pub struct AsyncClient {
    client: Arc<Mutex<ua::Client>>,
    background_handle: JoinHandle<()>,
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
                    log::debug!("Running iterate");

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
        }
    }

    /// Reads value from server.
    ///
    /// # Errors
    ///
    /// This fails when the node does not exist or its value attribute cannot be read.
    pub async fn read_value(&self, node_id: &ua::NodeId) -> Result<ua::DataValue, Error> {
        read_value(&self.client, node_id).await
    }

    /// Writes value from server.
    ///
    /// # Errors
    ///
    /// This fails when the node does not exist or its value attribute cannot be written.
    pub async fn write_value(
        &self,
        node_id: &ua::NodeId,
        value: &ua::DataValue,
    ) -> Result<(), Error> {
        let attribute_id = ua::AttributeId::value();

        let request = ua::WriteRequest::init().with_nodes_to_write(&[ua::WriteValue::init()
            .with_node_id(node_id)
            .with_attribute_id(&attribute_id)
            .with_value(value)]);

        let response = service_request(&self.client, request).await?;

        let Some(results) = response.results() else {
            return Err(Error::internal("write should return results"));
        };

        let Some(result) = results.as_slice().get(0) else {
            return Err(Error::internal("write should return a result"));
        };

        Error::verify_good(*result)?;

        Ok(())
    }

    /// Browses specific node.
    ///
    /// # Errors
    ///
    /// This fails when the node does not exist or it cannot be browsed.
    pub async fn browse(
        &self,
        node_id: &ua::NodeId,
    ) -> Result<Vec<ua::ReferenceDescription>, Error> {
        let request = ua::BrowseRequest::init()
            .with_nodes_to_browse(&[ua::BrowseDescription::default().with_node_id(node_id)]);

        let response = service_request(&self.client, request).await?;

        let Some(results) = response.results() else {
            return Err(Error::internal("browse should return results"));
        };

        let Some(result) = results.as_slice().get(0) else {
            return Err(Error::internal("browse should return a result"));
        };

        let Some(references) = result.references() else {
            return Err(Error::internal("browse should return references"));
        };

        Ok(references.as_slice().to_vec())
    }

    /// Browses several nodes at once.
    ///
    /// This issues only a single request to the OPC UA server (and should be preferred over several
    /// individual requests with [`browse()`] when browsing multiple nodes).
    ///
    /// # Errors
    ///
    /// This fails when the node does not exist or it cannot be browsed.
    ///
    /// [`browse()`]: Self::browse
    pub async fn browse_many(
        &self,
        node_ids: &[ua::NodeId],
    ) -> Result<Vec<Option<Vec<ua::ReferenceDescription>>>, Error> {
        let nodes_to_browse: Vec<_> = node_ids
            .iter()
            .map(|node_id| ua::BrowseDescription::default().with_node_id(node_id))
            .collect();

        let request = ua::BrowseRequest::init().with_nodes_to_browse(&nodes_to_browse);

        let response = service_request(&self.client, request).await?;

        let Some(results) = response.results() else {
            return Err(Error::internal("browse should return results"));
        };

        let results: Vec<_> = results
            .iter()
            .map(|result| {
                result
                    .references()
                    .map(|references| references.iter().cloned().collect())
            })
            .collect();
        debug_assert_eq!(results.len(), node_ids.len());

        Ok(results)
    }

    /// Creates new [subscription](AsyncSubscription).
    ///
    /// # Errors
    ///
    /// This fails when the client is not connected.
    pub async fn create_subscription(&self) -> Result<AsyncSubscription, Error> {
        AsyncSubscription::new(&self.client).await
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
    client: &Mutex<ua::Client>,
    node_id: &ua::NodeId,
) -> Result<ua::DataValue, Error> {
    type Cb = CallbackOnce<Result<ua::DataValue, ua::StatusCode>>;

    unsafe extern "C" fn callback_c(
        _client: *mut UA_Client,
        userdata: *mut c_void,
        _request_id: UA_UInt32,
        status: UA_StatusCode,
        value: *mut UA_DataValue,
    ) {
        log::debug!("readValueAttribute() completed");

        let status_code = ua::StatusCode::new(status);

        let result = if status_code.is_good() {
            // PANIC: We expect pointer to be valid when good.
            let value = value.as_ref().expect("value is set");
            Ok(ua::DataValue::clone_raw(value))
        } else {
            Err(status_code)
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

    let status_code = ua::StatusCode::new({
        let Ok(mut client) = client.lock() else {
            return Err(Error::internal("should be able to lock client"));
        };

        log::debug!("Calling readValueAttribute(), node_id={node_id:?}");

        unsafe {
            UA_Client_readValueAttribute_async(
                client.as_mut_ptr(),
                node_id.clone().into_raw(),
                Some(callback_c),
                Cb::prepare(callback),
                ptr::null_mut(),
            )
        }
    });
    Error::verify_good(status_code)?;

    // PANIC: When `callback` is called (which owns `tx`), we always call `tx.send()`. So the sender
    // is only dropped after placing a value into the channel and `rx.await` always finds this value
    // there.
    rx.await
        .unwrap_or(Err(Error::internal("callback should send result")))
}

async fn service_request<R: ServiceRequest>(
    client: &Mutex<ua::Client>,
    request: R,
) -> Result<R::Response, Error> {
    type Cb<R> = CallbackOnce<Result<<R as ServiceRequest>::Response, ua::StatusCode>>;

    unsafe extern "C" fn callback_c<R: ServiceRequest>(
        _client: *mut UA_Client,
        userdata: *mut c_void,
        _request_id: UA_UInt32,
        response: *mut c_void,
    ) {
        log::debug!("Request completed");

        // PANIC: We expect pointer to be valid when good.
        let response = R::Response::clone_raw(
            response
                .cast::<<R::Response as DataType>::Inner>()
                .as_ref()
                .expect("response is set"),
        );

        let status_code = response.service_result();
        let result = if status_code.is_good() {
            Ok(response)
        } else {
            Err(status_code)
        };

        Cb::<R>::execute(userdata, result);
    }

    let (tx, rx) = oneshot::channel::<Result<R::Response, Error>>();

    let callback = |result: Result<R::Response, _>| {
        // We always send a result back via `tx` (in fact, `rx.await` below expects this). We do not
        // care if that succeeds though: the receiver might already have gone out of scope (when its
        // future has been canceled) and we must not panic in FFI callbacks.
        let _unused = tx.send(result.map_err(Error::new));
    };

    let status_code = ua::StatusCode::new({
        let Ok(mut client) = client.lock() else {
            return Err(Error::internal("should be able to lock client"));
        };

        log::debug!("Calling request");

        unsafe {
            UA_Client_sendAsyncRequest(
                client.as_mut_ptr(),
                request.as_ptr().cast::<c_void>(),
                R::data_type(),
                Some(callback_c::<R>),
                R::Response::data_type(),
                Cb::<R>::prepare(callback),
                ptr::null_mut(),
            )
        }
    });
    Error::verify_good(status_code)?;

    // PANIC: When `callback` is called (which owns `tx`), we always call `tx.send()`. So the sender
    // is only dropped after placing a value into the channel and `rx.await` always finds this value
    // there.
    rx.await
        .unwrap_or(Err(Error::internal("callback should send result")))
}
