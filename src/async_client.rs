use std::{ffi::c_void, ptr, slice, sync::Arc, time::Duration};

use open62541_sys::{
    UA_Client, UA_Client_disconnectAsync, UA_Client_run_iterate, UA_UInt32,
    __UA_Client_AsyncService, UA_STATUSCODE_BADCONNECTIONCLOSED, UA_STATUSCODE_BADDISCONNECT,
};
use tokio::{
    sync::oneshot,
    task::JoinHandle,
    time::{self, Instant, MissedTickBehavior},
};

use crate::{
    ua, AsyncSubscription, CallbackOnce, ClientBuilder, DataType, Error, Result, ServiceRequest,
    ServiceResponse,
};

/// Connected OPC UA client (with asynchronous API).
///
/// To disconnect, prefer method [`disconnect()`](Self::disconnect) over simply dropping the client:
/// disconnection involves server communication and might take a short amount of time. If the client
/// is dropped when still connected, it will _synchronously_ clean up after itself, thereby blocking
/// while being dropped. In most cases, this is not the desired behavior.
///
/// See [Client](crate::Client) for more details.
pub struct AsyncClient {
    client: Arc<ua::Client>,
    background_handle: Option<JoinHandle<()>>,
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
    /// See [`ClientBuilder::connect()`] and [`Client::into_async()`](crate::Client::into_async).
    ///
    /// # Panics
    ///
    /// See [`ClientBuilder::connect()`].
    pub fn new(endpoint_url: &str, cycle_time: Duration) -> Result<Self> {
        Ok(ClientBuilder::default()
            .connect(endpoint_url)?
            .into_async(cycle_time))
    }

    pub(crate) fn from_sync(client: ua::Client, cycle_time: Duration) -> Self {
        let client = Arc::new(client);

        let background_task = background_task(Arc::clone(&client), cycle_time);
        // Run the event loop concurrently. This may be a different thread when using tokio with
        // `rt-multi-thread`.
        let background_handle = tokio::spawn(background_task);

        Self {
            client,
            background_handle: Some(background_handle),
        }
    }

    /// Gets current channel and session state, and connect status.
    #[must_use]
    pub fn state(&self) -> ua::ClientState {
        self.client.state()
    }

    /// Disconnects from endpoint.
    ///
    /// This consumes the client and handles the graceful shutdown of the connection. This should be
    /// preferred over simply dropping the instance to give the server a chance to clean up and also
    /// to avoid blocking unexpectedly when the client is being dropped without calling this method.
    pub async fn disconnect(mut self) {
        log::info!("Disconnecting from endpoint");

        let status_code = ua::StatusCode::new(unsafe {
            UA_Client_disconnectAsync(
                // SAFETY: Cast to `mut` pointer, function is marked `UA_THREADSAFE`.
                self.client.as_ptr().cast_mut(),
            )
        });
        if let Err(error) = Error::verify_good(&status_code) {
            log::warn!("Error while disconnecting client: {error}");
        }

        // Wait until background task has finished. The `take()` should always succeed because there
        // is no other location where the handle is being consumed.
        if let Some(background_handle) = self.background_handle.take() {
            let _unused = background_handle.await;
        }
    }

    /// Reads node value.
    ///
    /// To read other attributes, see [`read_attribute()`], [`read_attributes()`], and
    /// [`read_many_attributes()`].
    ///
    /// # Errors
    ///
    /// This fails when the node does not exist or its value attribute cannot be read.
    ///
    /// [`read_attribute()`]: Self::read_attribute
    /// [`read_attributes()`]: Self::read_attributes
    /// [`read_many_attributes()`]: Self::read_many_attributes
    pub async fn read_value(&self, node_id: &ua::NodeId) -> Result<ua::DataValue> {
        self.read_attribute(node_id, &ua::AttributeId::VALUE).await
    }

    /// Reads node attribute.
    ///
    /// To read only the value attribute, you can also use [`read_value()`].
    ///
    /// # Errors
    ///
    /// This fails when the node does not exist or the attribute cannot be read.
    ///
    /// [`read_value()`]: Self::read_value
    // TODO: Return inner `ua::Variant` instead of `ua::DataValue`.
    #[allow(clippy::missing_panics_doc)]
    pub async fn read_attribute(
        &self,
        node_id: &ua::NodeId,
        attribute_id: &ua::AttributeId,
    ) -> Result<ua::DataValue> {
        let mut values = self
            .read_attributes(node_id, slice::from_ref(attribute_id))
            .await?;

        // ERROR: We give a slice with one item to `read_attributes()` and expect a single result
        // value.
        debug_assert_eq!(values.len(), 1);
        values.pop().expect("should contain exactly one attribute")
    }

    /// Reads several node attributes.
    ///
    /// The size and order of the result list matches the size and order of the given attribute ID
    /// list.
    ///
    /// To read only a single attribute, you can also use [`read_attribute()`].
    ///
    /// # Errors
    ///
    /// This fails only when the entire request fails. When the node does not exist or one of the
    /// attributes cannot be read, an inner `Err` is returned.
    ///
    /// [`read_attribute()`]: Self::read_attribute
    // TODO: Return inner `ua::Variant` instead of `ua::DataValue`.
    pub async fn read_attributes(
        &self,
        node_id: &ua::NodeId,
        attribute_ids: &[ua::AttributeId],
    ) -> Result<Vec<Result<ua::DataValue>>> {
        // TODO: Avoid cloning, use `AsRef` in `read_many_attributes()`?
        self.read_many_attributes(
            &attribute_ids
                .iter()
                .map(|attribute_id| (node_id.clone(), attribute_id.clone()))
                .collect::<Vec<_>>(),
        )
        .await
    }

    /// Reads a combination of node attributes.
    ///
    /// The size and order of the result list matches the size and order of the given node ID and
    /// attribute ID list.
    ///
    /// To read attributes of a single node, you can also use [`read_attributes()`].
    ///
    /// # Errors
    ///
    /// This fails only when the entire request fails. When a node does not exist or one of the
    /// attributes cannot be read, an inner `Err` is returned.
    ///
    /// [`read_attributes()`]: Self::read_attributes
    // TODO: Return inner `ua::Variant` instead of `ua::DataValue`.
    pub async fn read_many_attributes(
        &self,
        node_attributes: &[(ua::NodeId, ua::AttributeId)],
    ) -> Result<Vec<Result<ua::DataValue>>> {
        let nodes_to_read: Vec<_> = node_attributes
            .iter()
            .map(|(node_id, attribute_id)| {
                ua::ReadValueId::init()
                    .with_node_id(node_id)
                    .with_attribute_id(attribute_id)
            })
            .collect();

        let request = ua::ReadRequest::init().with_nodes_to_read(&nodes_to_read);

        let response = service_request(&self.client, request).await?;

        let Some(results) = response.results() else {
            return Err(Error::internal("read should return results"));
        };

        let results: Vec<_> = results
            .iter()
            .map(|result| -> Result<ua::DataValue> {
                // An unset status code is considered valid: servers are not required to include the
                // status code in their response when not necessary.
                Error::verify_good(&result.status_code().unwrap_or(ua::StatusCode::GOOD))?;

                Ok(result.clone())
            })
            .collect();

        // The OPC UA specification state that the resulting list has the same number of elements as
        // the request list. If not, we would not be able to match elements in the two lists anyway.
        debug_assert_eq!(results.len(), node_attributes.len());

        Ok(results)
    }

    /// Writes node value.
    ///
    /// # Errors
    ///
    /// This fails when the node does not exist or its value attribute cannot be written.
    pub async fn write_value(&self, node_id: &ua::NodeId, value: &ua::DataValue) -> Result<()> {
        let attribute_id = ua::AttributeId::VALUE;

        let request = ua::WriteRequest::init().with_nodes_to_write(&[ua::WriteValue::init()
            .with_node_id(node_id)
            .with_attribute_id(&attribute_id)
            .with_value(value)]);

        let response = service_request(&self.client, request).await?;

        let Some(results) = response.results() else {
            return Err(Error::internal("write should return results"));
        };

        let Some(result) = results.as_slice().first() else {
            return Err(Error::internal("write should return a result"));
        };

        Error::verify_good(result)?;

        Ok(())
    }

    /// Calls specific method node at object node.
    ///
    /// # Errors
    ///
    /// This fails when the object or method node does not exist, the method cannot be called, or
    /// the input arguments are unexpected.
    pub async fn call_method(
        &self,
        object_id: &ua::NodeId,
        method_id: &ua::NodeId,
        input_arguments: &[ua::Variant],
    ) -> Result<Vec<ua::Variant>> {
        let request =
            ua::CallRequest::init().with_methods_to_call(&[ua::CallMethodRequest::init()
                .with_object_id(object_id)
                .with_method_id(method_id)
                .with_input_arguments(input_arguments)]);

        let response = service_request(&self.client, request).await?;

        let Some(results) = response.results() else {
            return Err(Error::internal("call should return results"));
        };

        let Some(result) = results.as_slice().first() else {
            return Err(Error::internal("call should return a result"));
        };

        Error::verify_good(&result.status_code())?;

        let output_arguments = if let Some(output_arguments) = result.output_arguments() {
            output_arguments.into_vec()
        } else {
            log::debug!("Calling {method_id} returned unset output arguments, assuming none exist");
            Vec::new()
        };

        Ok(output_arguments)
    }

    /// Browses specific node.
    ///
    /// # Errors
    ///
    /// This fails when the node does not exist or it cannot be browsed.
    pub async fn browse(&self, node_id: &ua::NodeId) -> BrowseResult {
        let request = ua::BrowseRequest::init()
            .with_nodes_to_browse(&[ua::BrowseDescription::default().with_node_id(node_id)]);

        let response = service_request(&self.client, request).await?;

        let Some(results) = response.results() else {
            return Err(Error::internal("browse should return results"));
        };

        let Some(result) = results.as_slice().first() else {
            return Err(Error::internal("browse should return a result"));
        };

        to_browse_result(result, Some(node_id))
    }

    /// Browses several nodes at once.
    ///
    /// This issues only a single request to the OPC UA server (and should be preferred over several
    /// individual requests with [`browse()`] when browsing multiple nodes).
    ///
    /// The size and order of the result list matches the size and order of the given node ID list.
    ///
    /// # Errors
    ///
    /// This fails only when the entire request fails. When a node does not exist or cannot be
    /// browsed, an inner `Err` is returned.
    ///
    /// [`browse()`]: Self::browse
    pub async fn browse_many(&self, node_ids: &[ua::NodeId]) -> Result<Vec<BrowseResult>> {
        let nodes_to_browse: Vec<_> = node_ids
            .iter()
            .map(|node_id| ua::BrowseDescription::default().with_node_id(node_id))
            .collect();

        let request = ua::BrowseRequest::init().with_nodes_to_browse(&nodes_to_browse);

        let response = service_request(&self.client, request).await?;

        let Some(results) = response.results() else {
            return Err(Error::internal("browse should return results"));
        };

        // The OPC UA specification state that the resulting list has the same number of elements as
        // the request list. If not, we would not be able to match elements in the two lists anyway.
        debug_assert_eq!(results.len(), node_ids.len());

        let results: Vec<_> = results
            .iter()
            .zip(node_ids)
            .map(|(result, node_id)| to_browse_result(result, Some(node_id)))
            .collect();

        Ok(results)
    }

    /// Browses continuation points for more references.
    ///
    /// This uses continuation points returned from [`browse()`] and [`browse_many()`] whenever not
    /// all references were returned (due to client or server limits).
    ///
    /// The size and order of the result list matches the size and order of the given continuation
    /// point list.
    ///
    /// # Errors
    ///
    /// This fails only when the entire request fails. When a continuation point is invalid, an
    /// inner `Err` is returned.
    ///
    /// [`browse()`]: Self::browse
    /// [`browse_many()`]: Self::browse_many
    pub async fn browse_next(
        &self,
        continuation_points: &[ua::ContinuationPoint],
    ) -> Result<Vec<BrowseResult>> {
        let request = ua::BrowseNextRequest::init().with_continuation_points(continuation_points);

        let response = service_request(&self.client, request).await?;

        let Some(results) = response.results() else {
            return Err(Error::internal("browse should return results"));
        };

        // The OPC UA specification state that the resulting list has the same number of elements as
        // the request list. If not, we would not be able to match elements in the two lists anyway.
        debug_assert_eq!(results.len(), continuation_points.len());

        let results: Vec<_> = results
            .iter()
            .map(|result| to_browse_result(result, None))
            .collect();

        Ok(results)
    }

    /// Creates new [subscription](AsyncSubscription).
    ///
    /// # Errors
    ///
    /// This fails when the client is not connected.
    pub async fn create_subscription(&self) -> Result<AsyncSubscription> {
        AsyncSubscription::new(&self.client).await
    }
}

impl Drop for AsyncClient {
    fn drop(&mut self) {
        // The background task handle may already have been consumed in `disconnect()`. If so, there
        // is nothing we need to do here, the task is already being run to completion (even when the
        // method call might have been canceled).
        if let Some(background_handle) = self.background_handle.take() {
            // Abort background task (at the interval await point).
            background_handle.abort();
        }
    }
}

async fn background_task(client: Arc<ua::Client>, cycle_time: Duration) {
    log::debug!("Starting background task");

    let mut interval = time::interval(cycle_time);
    // TODO: Offer customized `MissedTickBehavior`? Only `Skip` and `Delay` are suitable here as we
    // don't want `Burst` to repeatedly and unnecessarily call `UA_Client_run_iterate()` many times
    // in a row.
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

    // `UA_Client_run_iterate()` must be run periodically and makes sure to maintain the connection
    // (e.g. renew session) and run callback handlers.
    loop {
        // This await point is where the background task could be aborted. (The first tick finishes
        // immediately, so there is no additional delay on the first iteration.)
        interval.tick().await;
        // Track time of cycle start to report missed cycles below.
        let start_of_cycle = Instant::now();

        let status_code = ua::StatusCode::new({
            log::trace!("Running iterate");

            // Timeout of 0 means we do not block here at all. We don't want to hold the mutex
            // longer than necessary (because that would block requests from being sent out).
            // TODO: Re-evaluate this.
            unsafe {
                UA_Client_run_iterate(
                    // SAFETY: Cast to `mut` pointer, function is marked `UA_THREADSAFE`.
                    client.as_ptr().cast_mut(),
                    0,
                )
            }
        });
        if let Err(error) = Error::verify_good(&status_code) {
            // Context-sensitive handling of bad status codes.
            match status_code.into_raw() {
                UA_STATUSCODE_BADDISCONNECT => {
                    // Not an error.
                    log::info!("Terminating background task after disconnect");
                }
                UA_STATUSCODE_BADCONNECTIONCLOSED => {
                    // Not an error.
                    log::info!("Terminating background task after connection closed");
                }
                _ => {
                    // Unexpected error.
                    log::error!("Terminating background task: Run iterate failed with {error}");
                }
            }
            return;
        }

        let time_taken = start_of_cycle.elapsed();

        // Detect and log missed cycles.
        if !cycle_time.is_zero() && time_taken > cycle_time {
            let missed_cycles = time_taken.as_nanos() / cycle_time.as_nanos();
            log::warn!("Iterate run took {time_taken:?}, missed {missed_cycles} cycle(s)");
        } else {
            log::trace!("Iterate run took {time_taken:?}");
        }
    }
}

async fn service_request<R: ServiceRequest>(
    client: &ua::Client,
    request: R,
) -> Result<R::Response> {
    type Cb<R> = CallbackOnce<std::result::Result<<R as ServiceRequest>::Response, ua::StatusCode>>;

    unsafe extern "C" fn callback_c<R: ServiceRequest>(
        _client: *mut UA_Client,
        userdata: *mut c_void,
        request_id: UA_UInt32,
        response: *mut c_void,
    ) {
        log::trace!(
            "Request ID {request_id} finished, received {}",
            R::Response::type_name(),
        );

        // SAFETY: Incoming pointer is valid for access.
        // PANIC: We expect pointer to be valid when good.
        let response = unsafe { response.cast::<<R::Response as DataType>::Inner>().as_ref() }
            .expect("response should be set");
        let response = R::Response::clone_raw(response);

        let status_code = response.service_result();
        let result = if status_code.is_good() {
            Ok(response)
        } else {
            Err(status_code)
        };

        // SAFETY: `userdata` is the result of `Cb::prepare()` and is used only once.
        unsafe {
            Cb::<R>::execute(userdata, result);
        }
    }

    let (tx, rx) = oneshot::channel::<Result<R::Response>>();

    let callback = |result: std::result::Result<R::Response, _>| {
        // We always send a result back via `tx` (in fact, `rx.await` below expects this). We do not
        // care if that succeeds though: the receiver might already have gone out of scope (when its
        // future has been canceled) and we must not panic in FFI callbacks.
        let _unused = tx.send(result.map_err(Error::new));
    };

    log::debug!("Running {}", R::type_name());

    let mut request_id: UA_UInt32 = 0;
    let status_code = ua::StatusCode::new(unsafe {
        __UA_Client_AsyncService(
            // SAFETY: Cast to `mut` pointer, function is marked `UA_THREADSAFE`.
            client.as_ptr().cast_mut(),
            request.as_ptr().cast::<c_void>(),
            R::data_type(),
            Some(callback_c::<R>),
            R::Response::data_type(),
            Cb::<R>::prepare(callback),
            ptr::addr_of_mut!(request_id),
        )
    });
    // The request itself fails when the client is not connected (or the secure session has not been
    // established). In all other cases, `open62541` processes the request first and then may reject
    // it only through the response when executing our callback above.
    Error::verify_good(&status_code).inspect_err(|_| {
        log::warn!("{} failed: {status_code:?}", R::type_name());
    })?;

    log::trace!("Assigned ID {request_id} to {}", R::type_name());

    // PANIC: When `callback` is called (which owns `tx`), we always call `tx.send()`. So the sender
    // is only dropped after placing a value into the channel and `rx.await` always finds this value
    // there.
    rx.await
        .unwrap_or(Err(Error::internal("callback should send result")))
}

/// Result type for browsing.
pub type BrowseResult = Result<(Vec<ua::ReferenceDescription>, Option<ua::ContinuationPoint>)>;

/// Converts [`ua::BrowseResult`] to our public result type.
fn to_browse_result(result: &ua::BrowseResult, node_id: Option<&ua::NodeId>) -> BrowseResult {
    // Make sure to verify the inner status code inside `BrowseResult`. The service request finishes
    // without error, even when browsing the node has failed.
    Error::verify_good(&result.status_code())?;

    let references = if let Some(references) = result.references() {
        references.into_vec()
    } else {
        // When no references exist, some OPC UA servers do not return an empty references array but
        // an invalid (unset) one instead, e.g. Siemens SIMOTION. We treat it as an empty array, and
        // continue without error.
        if let Some(node_id) = node_id {
            log::debug!("Browsing {node_id} returned unset references, assuming none exist");
        } else {
            log::debug!(
                "Browsing continuation point returned unset references, assuming none exist",
            );
        }
        Vec::new()
    };

    Ok((references, result.continuation_point()))
}
