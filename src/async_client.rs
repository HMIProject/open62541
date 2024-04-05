use std::{
    ffi::c_void,
    ptr, slice,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use open62541_sys::{
    UA_Client, UA_Client_disconnectAsync, UA_Client_run_iterate, UA_UInt32,
    __UA_Client_AsyncService, UA_STATUSCODE_BADCONNECTIONCLOSED, UA_STATUSCODE_BADDISCONNECT,
};
use tokio::{sync::oneshot, task, time::Instant};

use crate::{
    ua, AsyncSubscription, CallbackOnce, ClientBuilder, DataType, Error, Result, ServiceRequest,
    ServiceResponse,
};

/// Timeout for `UA_Client_run_iterate()`.
///
/// This is the maximum amount of time that `UA_Client_run_iterate()` will block for. It is relevant
/// primarily when canceling the background task, i.e. when we need to interrupt the loop and cancel
/// before the next invocation of `UA_Client_run_iterate()`.
///
/// Since this is also the timeout we must block for when dropping the client without `disconnect()`
/// first, the value should not be too large. On the other hand, it should not be too small to avoid
/// repeatedly calling `poll()`/`select()` inside open62541's event loop implementation.
const RUN_ITERATE_TIMEOUT: Duration = Duration::from_millis(200);

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
    background_canceled: Arc<AtomicBool>,
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
    pub fn new(endpoint_url: &str) -> Result<Self> {
        Ok(ClientBuilder::default().connect(endpoint_url)?.into_async())
    }

    pub(crate) fn from_sync(client: ua::Client) -> Self {
        let client = Arc::new(client);

        let background_canceled = Arc::new(AtomicBool::new(false));

        // Run the event loop concurrently. We do so on a thread where we may block: we need to call
        // `UA_Client_run_iterate()` and this method blocks for up to `RUN_ITERATE_TIMEOUT`.
        //
        // We use an OS thread here instead of tokio's blocking tasks because we may need to join on
        // the task blockingly in `drop()` and this requires proper concurrency (otherwise, we would
        // risk deadlocking on single-threaded tokio runners).
        let background_handle = {
            let client = Arc::clone(&client);
            let canceled = Arc::clone(&background_canceled);
            thread::spawn(move || background_task(&client, &canceled))
        };

        Self {
            client,
            background_canceled,
            background_handle: Some(background_handle),
        }
    }

    /// Waits for background task to finish.
    ///
    /// Note: This _blocks_ the current thread while waiting for the thread that runs the background
    /// task to finish. Either use `cancel` to set the cancellation token or make sure to disconnect
    /// client first so that the task eventually finishes on its own.
    fn join_background_task(&mut self, cancel: bool) {
        // We only take the handle when we join. So if handle has already been taken, the background
        // task is not running anymore. This usually happens in `drop()` after `disconnect()`.
        let Some(background_handle) = self.background_handle.take() else {
            return;
        };

        if cancel {
            log::info!("Canceling background task");
            self.background_canceled.store(true, Ordering::Relaxed);
        }

        // TODO: Use `tracing` and span to group log messages.
        log::info!("Waiting for background task to finish");

        // This call blocks. We ignore the result because we do not care if the thread panicked (and
        // there is nothing that we could do anyway in that case).
        let _unused = background_handle.join();

        log::info!("Background task finished");
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

        // Wait for background task to complete. Since `join_background_task()` blocks, we must wait
        // in a separate tokio task. We ignore the result (since we do not care if the task panicked
        // and there is nothing else it returns).
        //
        // Note: We do _not_ cancel the background task before blocking: we require the asynchronous
        // handling to keep on running until the connection has been taken down which then makes the
        // task finish by itself.
        let _unused = task::spawn_blocking(move || self.join_background_task(false)).await;
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
        // We need to wait for the task to finish and must do so blockingly. `UA_Client_delete()` is
        // not safe run concurrently while `UA_Client_run_iterate()` is still running.
        //
        // Notify background task to cancel itself, even when [`UA_Client_run_iterate()`] would want
        // to keep on running. This is okay: we are not issuing asynchronous requests anymore anyway
        // (the only other call will be `UA_Client_delete()` when inner client drops).
        self.join_background_task(true);
    }
}

/// Background task for [`ua::Client`].
///
/// This runs [`UA_Client_run_iterate()`] in a loop, blocking for up to `RUN_ITERATE_TIMEOUT` during
/// each iteration. In case the loop does not finish by itself (which happens in case of disconnects
/// and for final connection failures), the cancellation token `cancel` can be used to stop the task
/// from the outside before the next loop iteration.
fn background_task(client: &ua::Client, canceled: &AtomicBool) {
    log::info!("Starting background task");

    // `UA_Client_run_iterate()` expects the timeout to be given in milliseconds.
    let timeout_millis = u32::try_from(RUN_ITERATE_TIMEOUT.as_millis()).unwrap_or(u32::MAX);

    // Run until canceled. The only other way to exit is when `UA_Client_run_iterate()` itself fails
    // (which happens when the connection is broken and the client instance cannot be used anymore).
    while !canceled.load(Ordering::Relaxed) {
        // Track time of iteration start to report iteration times below.
        let start_of_iteration = Instant::now();

        let status_code = ua::StatusCode::new({
            log::trace!("Running iterate");

            // This returns after the timeout even when nothing was processed. The internal mutex is
            // _not_ held for the entire time though, so we can send out requests concurrently while
            // the client is running the iteration.
            unsafe {
                UA_Client_run_iterate(
                    // SAFETY: Cast to `mut` pointer, function is marked `UA_THREADSAFE`.
                    client.as_ptr().cast_mut(),
                    timeout_millis,
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
                    log::error!("Terminating background task: run failed with {error}");
                }
            }
            return;
        }

        let time_taken = start_of_iteration.elapsed();
        log::trace!("Iterate run took {time_taken:?}");
    }

    log::info!("Terminating canceled background task");
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
