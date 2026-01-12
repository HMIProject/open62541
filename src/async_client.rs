use std::{
    collections::{BTreeMap, BTreeSet, HashSet, LinkedList, VecDeque},
    ffi::c_void,
    slice,
    sync::{
        Arc, Weak,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use futures_channel::oneshot;
use open62541_sys::{
    __UA_Client_AsyncService, UA_Client, UA_Client_disconnectAsync, UA_Client_run_iterate,
    UA_STATUSCODE_BADCONNECTIONCLOSED, UA_STATUSCODE_BADDISCONNECT, UA_UInt32,
};

use crate::{
    AsyncSubscription, Attribute, BrowseResult, CallbackOnce, DataType, DataValue, Error, Result,
    ServiceRequest, ServiceResponse, SubscriptionBuilder, ua,
};

/// Timeout for `UA_Client_run_iterate()`.
///
/// This is the maximum amount of time that `UA_Client_run_iterate()` will block for. It is relevant
/// primarily when cancelling the background task, i.e. when we interrupt the loop and cancel before
/// the next invocation of `UA_Client_run_iterate()`.
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
/// With feature `tokio` enabled, blocking invocations in [`AsyncClient::drop()`] might be offloaded
/// from executor to worker threads as needed to prevent deadlocks. However, this can be implemented
/// only when running in [multi-threaded runtimes]. When using current-thread runtime (or some other
/// asynchronous runtime), make sure to not invoke [`AsyncClient::drop()`] in asynchronous contexts.
///
/// See [Client](crate::Client) for more details.
///
/// [multi-threaded runtimes]: https://docs.rs/tokio/latest/tokio/runtime/index.html
#[derive(Debug)]
pub struct AsyncClient {
    client: Arc<ua::Client>,
    background_thread: Option<BackgroundThread>,
    known_data_type_types: BTreeMap<ua::NodeId, ua::DataType>,
}

impl AsyncClient {
    /// Creates default client connected to endpoint.
    ///
    /// If you need more control over the initialization, use [`ClientBuilder`] instead, and turn it
    /// into [`Client`](crate::Client) by calling [`connect()`](crate::ClientBuilder::connect), then
    /// follow this with [`into_async()`](crate::Client::into_async) to get the asynchronous API.
    ///
    /// # Errors
    ///
    /// See [`ClientBuilder::connect()`] and [`Client::into_async()`](crate::Client::into_async).
    ///
    /// # Panics
    ///
    /// See [`ClientBuilder::connect()`].
    ///
    /// [`ClientBuilder`]: crate::ClientBuilder
    /// [`ClientBuilder::connect()`]: crate::ClientBuilder::connect
    pub fn new(endpoint_url: &str) -> Result<Self> {
        Ok(crate::Client::new(endpoint_url)?.into_async())
    }

    pub(crate) fn from_sync(client: ua::Client) -> Self {
        let client = Arc::new(client);
        let background_thread = BackgroundThread::spawn(Arc::clone(&client));
        Self {
            client,
            background_thread: Some(background_thread),
            known_data_type_types: BTreeMap::new(),
        }
    }

    pub(crate) fn upgrade_weak(client: &Weak<ua::Client>) -> Result<Arc<ua::Client>> {
        client
            .upgrade()
            .ok_or(Error::internal("client has been dropped"))
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
    #[expect(clippy::missing_panics_doc, reason = "implementation invariant")]
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

        // PANIC: We only take the background thread in this method. Since it consumes `self`, the value must
        // still be present when we reach this. Do so only right before awaiting to uphold invariant
        // in `Drop` implementation which allows us to take an early return path there.
        let background_thread = self.background_thread.take().expect("no background thread");

        // Asynchronously wait for the background task running in the background thread to complete.
        //
        // Note: We do _not_ cancel the background task before blocking: we require the asynchronous
        // handling to keep on running until the connection has been taken down which then makes the
        // task finish by itself.
        background_thread.wait_until_done().await;
    }

    pub fn add_data_types(
        &self,
        data_type_descriptions: &[ua::DataTypeDescription],
    ) -> Result<usize> {
        // Pick only descriptions from incoming slice that we do not already know. The reason is: we
        // do not know whether these data types have already been referred to by _other_ data types;
        // so, for each data type ID, we must always use the same compiled data type instance.
        let new_data_type_descriptions = data_type_descriptions
            .iter()
            .filter_map(|data_type_description| {
                let data_type_id = data_type_description.data_type_id();
                if is_well_known_data_type(data_type_id)
                    || self.known_data_type_types.contains_key(data_type_id)
                {
                    return None;
                }
                Some((data_type_id, data_type_description))
            })
            .collect::<BTreeMap<_, _>>();
        let new_data_type_ids = new_data_type_descriptions
            .keys()
            .copied()
            .cloned()
            .collect::<Vec<_>>();

        println!("<= {new_data_type_ids:?}");

        // Find dependency order: `sorted_data_type_ids` will contain dependants before dependencies
        // (e.g. data type for structure followed by data types for its fields). Each data type (ID)
        // will be listed only once and will not depend on any other data types listed before it.
        let Some(sorted_data_type_ids) = topological_sort(&new_data_type_ids, |data_type_id| {
            let data_type_description = new_data_type_descriptions.get(data_type_id).unwrap();
            match data_type_description {
                ua::DataTypeDescription::Structure(description) => {
                    let Some(fields) = description.structure_definition().fields() else {
                        return BTreeSet::new();
                    };
                    fields
                        .iter()
                        .filter_map(|field| {
                            let data_type = field.data_type();
                            new_data_type_descriptions
                                .contains_key(data_type)
                                .then(|| data_type.to_owned())
                        })
                        .collect()
                }
                ua::DataTypeDescription::Enum(_) => BTreeSet::new(),
            }
        }) else {
            return Err(Error::Internal("cyclical data type descriptions"));
        };

        println!("=> {sorted_data_type_ids:?}");

        let mut new_data_types = Vec::with_capacity(sorted_data_type_ids.len());

        for data_type_id in sorted_data_type_ids {
            let &data_type_description = new_data_type_descriptions
                .get(&data_type_id)
                .expect("data type description exists");

            let data_type = data_type_description
                .to_data_type(Some(&ua::DataTypeArray::new(&mut new_data_types)))?;

            new_data_types.push(data_type.into_raw());
        }

        Ok(0)
    }

    /// Reads node value.
    ///
    /// To read other attributes, see [`read_attribute()`], [`read_attributes()`], and
    /// [`read_many_attributes()`].
    ///
    /// # Errors
    ///
    /// This fails only when the entire request fails (e.g. communication error). When the node does
    /// not exist or its value attribute cannot be read, the server returns a [`DataValue`] with the
    /// appropriate [`status()`] and with [`value()`] unset.
    ///
    /// [`read_attribute()`]: Self::read_attribute
    /// [`read_attributes()`]: Self::read_attributes
    /// [`read_many_attributes()`]: Self::read_many_attributes
    /// [`status()`]: DataValue::status
    /// [`value()`]: DataValue::value
    pub async fn read_value(&self, node_id: &ua::NodeId) -> Result<DataValue<ua::Variant>> {
        self.read_attribute(node_id, ua::AttributeId::VALUE_T).await
    }

    /// Reads node attribute.
    ///
    /// To read only the value attribute, you can also use [`read_value()`].
    ///
    /// # Errors
    ///
    /// This fails only when the entire request fails (e.g. communication error). When the node does
    /// not exist or the given attribute cannot be read, the server returns a [`DataValue`] with the
    /// appropriate [`status()`] and with [`value()`] unset.
    ///
    /// [`read_value()`]: Self::read_value
    /// [`status()`]: DataValue::status
    /// [`value()`]: DataValue::value
    pub async fn read_attribute<T: Attribute>(
        &self,
        node_id: &ua::NodeId,
        attribute: T,
    ) -> Result<DataValue<T::Value>> {
        let mut values = self.read_attributes(node_id, &[attribute.id()]).await?;

        // ERROR: We give a slice with one item to `read_attributes()` and expect a single result
        // value.
        debug_assert_eq!(values.len(), 1);
        let Some(value) = values.pop() else {
            return Err(Error::internal("should contain exactly one attribute"));
        };

        Ok(value.cast())
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
    /// This fails only when the entire request fails (e.g. communication error). When the node does
    /// not exist or one of the given attributes cannot be read, the server returns a corresponding
    /// [`DataValue`] with the appropriate [`status()`] and with [`value()`] unset.
    ///
    /// [`read_attribute()`]: Self::read_attribute
    /// [`status()`]: DataValue::status
    /// [`value()`]: DataValue::value
    pub async fn read_attributes(
        &self,
        node_id: &ua::NodeId,
        attribute_ids: &[ua::AttributeId],
    ) -> Result<Vec<DataValue<ua::Variant>>> {
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
    /// This fails only when the entire request fails (e.g. communication error). When a node does
    /// not exist or one of the given attributes cannot be read, the server returns a corresponding
    /// [`DataValue`] with the appropriate [`status()`] and with [`value()`] unset.
    ///
    /// [`read_attributes()`]: Self::read_attributes
    /// [`status()`]: DataValue::status
    /// [`value()`]: DataValue::value
    pub async fn read_many_attributes(
        &self,
        node_attributes: &[(ua::NodeId, ua::AttributeId)],
    ) -> Result<Vec<DataValue<ua::Variant>>> {
        let nodes_to_read: Vec<_> = node_attributes
            .iter()
            .map(|(node_id, attribute_id)| {
                ua::ReadValueId::init()
                    .with_node_id(node_id)
                    .with_attribute_id(attribute_id)
            })
            .collect();

        let request = ua::ReadRequest::init()
            // TODO: Add method argument for this? We return timestamps in `DataValue` and they
            // should not end up always being `None` by default.
            .with_timestamps_to_return(&ua::TimestampsToReturn::BOTH)
            .with_nodes_to_read(&nodes_to_read);

        let response = self.service_request(request).await?;

        let Some(mut results) = response.results() else {
            return Err(Error::internal("read should return results"));
        };

        let results: Vec<DataValue<ua::Variant>> =
            results.drain_all().map(ua::DataValue::cast).collect();

        // The OPC UA specification state that the resulting list has the same number of elements as
        // the request list. If not, we would not be able to match elements in the two lists anyway.
        if results.len() != node_attributes.len() {
            return Err(Error::internal("unexpected number of read results"));
        }

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

        let response = self.service_request(request).await?;

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

        let response = self.service_request(request).await?;

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
    /// Use [`ua::BrowseDescription::default()`](ua::BrowseDescription) to set sensible defaults to
    /// browse a specific node's children (forward references of the `HierarchicalReferences` type)
    /// like this:
    ///
    /// ```
    /// # use open62541::{AsyncClient, Result, ua};
    /// use open62541_sys::UA_NS0ID_SERVER_SERVERSTATUS;
    ///
    /// # async fn example(client: &AsyncClient) -> Result<()> {
    /// let node_id = ua::NodeId::ns0(UA_NS0ID_SERVER_SERVERSTATUS);
    /// let browse_description = ua::BrowseDescription::default().with_node_id(&node_id);
    /// let (references, continuation_point) = client.browse(&browse_description).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This fails when the node does not exist or it cannot be browsed.
    pub async fn browse(&self, browse_description: &ua::BrowseDescription) -> BrowseResult {
        let request =
            ua::BrowseRequest::init().with_nodes_to_browse(slice::from_ref(browse_description));

        let response = self.service_request(request).await?;

        let Some(results) = response.results() else {
            return Err(Error::internal("browse should return results"));
        };

        let Some(result) = results.as_slice().first() else {
            return Err(Error::internal("browse should return a result"));
        };

        to_browse_result(result, Some(browse_description.node_id()))
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
    pub async fn browse_many(
        &self,
        browse_descriptions: &[ua::BrowseDescription],
    ) -> Result<Vec<BrowseResult>> {
        let request = ua::BrowseRequest::init().with_nodes_to_browse(browse_descriptions);

        let response = self.service_request(request).await?;

        let Some(results) = response.results() else {
            return Err(Error::internal("browse should return results"));
        };

        // The OPC UA specification state that the resulting list has the same number of elements as
        // the request list. If not, we would not be able to match elements in the two lists anyway.
        if results.len() != browse_descriptions.len() {
            return Err(Error::internal("unexpected number of browse results"));
        }

        let results: Vec<_> = results
            .iter()
            .zip(browse_descriptions)
            .map(|(result, browse_description)| {
                to_browse_result(result, Some(browse_description.node_id()))
            })
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

        let response = self.service_request(request).await?;

        let Some(results) = response.results() else {
            return Err(Error::internal("browse should return results"));
        };

        // The OPC UA specification state that the resulting list has the same number of elements as
        // the request list. If not, we would not be able to match elements in the two lists anyway.
        if results.len() != continuation_points.len() {
            return Err(Error::Internal("unexpected number of browse results"));
        }

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
        let (_, subscription) = SubscriptionBuilder::default().create(self).await?;

        Ok(subscription)
    }

    /// Services a generic request.
    ///
    /// Could be used with [`ua::ReadRequest`]/[`ua::WriteRequest`],
    /// [`ua::BrowseRequest`]/[`ua::BrowseNextRequest`], and [`ua::CallRequest`].
    ///
    /// # Errors
    ///
    /// This fails when the client is not connected.
    pub async fn service_request<R: ServiceRequest>(&self, request: R) -> Result<R::Response> {
        type Cb<R> =
            CallbackOnce<std::result::Result<<R as ServiceRequest>::Response, ua::StatusCode>>;

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

            let service_result = response.response_header().service_result();
            let result = if service_result.is_good() {
                Ok(response)
            } else {
                Err(service_result)
            };

            // SAFETY: `userdata` is the result of `Cb::prepare()` and is used only once.
            unsafe {
                Cb::<R>::execute(userdata, result);
            }
        }

        let (tx, rx) = oneshot::channel::<Result<R::Response>>();

        let callback = move |result: std::result::Result<R::Response, _>| {
            // We always send a result back via `tx` (in fact, `rx.await` below expects this). We do not
            // care if that succeeds though: the receiver might already have gone out of scope (when its
            // future has been cancelled) and we must not panic in FFI callbacks.
            let _unused = tx.send(result.map_err(Error::new));
        };

        log::debug!("Running {}", R::type_name());

        let mut request_id: UA_UInt32 = 0;
        let status_code = ua::StatusCode::new(unsafe {
            __UA_Client_AsyncService(
                // SAFETY: Cast to `mut` pointer, function is marked `UA_THREADSAFE`.
                self.client.as_ptr().cast_mut(),
                request.as_ptr().cast::<c_void>(),
                R::data_type(),
                Some(callback_c::<R>),
                R::Response::data_type(),
                Cb::<R>::prepare(callback),
                &raw mut request_id,
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

    pub(crate) const fn client(&self) -> &Arc<ua::Client> {
        &self.client
    }
}

impl Drop for AsyncClient {
    fn drop(&mut self) {
        // If `disconnect()` has been called before (either run to completion or cancelled), all has
        // been done to shut down the background thread. The only thing that may be left is properly
        // joining the background thread. We don't do that here to avoid the amount of blocking that
        // this could involve. By dropping the handle, the OS is able to release all resources soon,
        // i.e., when the thread has actually run to completion (if it hasn't done so already).
        let Some(background_thread) = self.background_thread.take() else {
            log::debug!("Background task has already finished before dropping client");
            return;
        };

        log::info!("Cancelling and joining background task when dropping client");
        background_thread.cancel_and_join();

        log::info!("Background task finished when dropping client");
    }
}

#[derive(Debug)]
struct BackgroundThread {
    cancelled: Arc<AtomicBool>,
    done_rx: oneshot::Receiver<()>,
    handle: JoinHandle<()>,
}

impl BackgroundThread {
    fn spawn(client: Arc<ua::Client>) -> Self {
        let cancelled = Arc::new(AtomicBool::new(false));
        let (done_tx, done_rx) = oneshot::channel();

        // Run the event loop concurrently. We do so on a thread where we may block: we need to call
        // `UA_Client_run_iterate()` and this method blocks for up to `RUN_ITERATE_TIMEOUT`.
        //
        // We use an OS thread here instead of tokio's blocking tasks because we may need to join on
        // the task blockingly in `drop()` and this requires proper concurrency (otherwise, we would
        // risk deadlocking on single-threaded tokio runners).
        let handle = {
            let cancelled = Arc::clone(&cancelled);
            thread::spawn(move || {
                background_task(&client, &cancelled);
                log::info!("Background task finished");
                let _unused = done_tx.send(());
            })
        };

        Self {
            cancelled,
            done_rx,
            handle,
        }
    }

    fn cancel_and_join(self) {
        let Self {
            cancelled, handle, ..
        } = self;

        // Notify background task to cancel itself, even when [`UA_Client_run_iterate()`] would want
        // to keep on running. This is okay: we are not issuing asynchronous requests anymore anyway
        // (the only other call will be `UA_Client_delete()` when inner client drops).
        cancelled.store(true, Ordering::Relaxed);

        // We need to wait for the task to finish and must do so blockingly. `UA_Client_delete()` is
        // not safe run concurrently while `UA_Client_run_iterate()` is still running. We ignore the
        // result, because we do not care if the thread panicked (and there is nothing that we could
        // do anyway in that case).
        // TODO: Use `tracing` and span to group log messages.
        log::info!("Waiting for background task to finish after cancelling");

        // `AsyncClient` is supposed to be used in asynchronous context. Note that blocking executor
        // threads may cause deadlocks and must be avoided.
        #[cfg(feature = "tokio")]
        if let Ok(rt) = &tokio::runtime::Handle::try_current() {
            if matches!(
                rt.runtime_flavor(),
                tokio::runtime::RuntimeFlavor::CurrentThread
            ) {
                // Do not spawn new thread, because we do not have multiple threads in this runtime.
                tokio::task::block_in_place(move || {
                    let _unused = handle.join();
                });
            } else {
                // Offload the synchronous invocation from the executor thread onto a worker thread.
                let join_handle = rt.spawn_blocking(move || {
                    let _unused = handle.join();
                });
                // Re-enter the asynchronous context for joining the worker thread.
                tokio::task::block_in_place(move || {
                    rt.block_on(async move {
                        let _unused = join_handle.await;
                    });
                });
            }
            return;
        }

        let _unused = handle.join();
    }

    async fn wait_until_done(self) {
        let Self { done_rx, .. } = self;

        // We ignore the result: the sender is only dropped when the background thread has finished,
        // which is exactly what we are waiting for anyway.
        let _unused = done_rx.await;
    }
}

/// Background task for [`ua::Client`].
///
/// This runs [`UA_Client_run_iterate()`] in a loop, blocking for up to `RUN_ITERATE_TIMEOUT` during
/// each iteration. In case the loop does not finish by itself (which happens in case of disconnects
/// and for final connection failures), the cancellation token `cancel` can be used to stop the task
/// from the outside before the next loop iteration.
fn background_task(client: &ua::Client, cancelled: &AtomicBool) {
    log::info!("Starting background task");

    // `UA_Client_run_iterate()` expects the timeout to be given in milliseconds.
    let timeout_millis = u32::try_from(RUN_ITERATE_TIMEOUT.as_millis()).unwrap_or(u32::MAX);

    // Run until cancelled. The only other way to exit is when `UA_Client_run_iterate()` fails which
    // happens when the connection is broken and the client instance cannot be used anymore.
    while !cancelled.load(Ordering::Relaxed) {
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

    log::info!("Terminating cancelled background task");
}

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

fn topological_sort<T, F>(nodes: &[T], mut get_deps: F) -> Option<Vec<T>>
where
    T: Clone + Ord,
    F: FnMut(&T) -> BTreeSet<T>,
{
    // Create list of edges from each node to its respective dependencies.
    let mut pending_deps = nodes
        .iter()
        .map(|a| (a, get_deps(a)))
        .collect::<BTreeMap<_, _>>();

    // Prepare eventual result, and starting set for algorithm: any nodes without dependencies.
    let mut result = Vec::with_capacity(nodes.len());
    let mut pending_nodes = nodes
        .iter()
        .filter(|&a| pending_deps.get(a).expect("node exists").is_empty())
        .collect::<Vec<_>>();

    // Iterate over nodes without dependencies until all (reachable) nodes have been visited. During
    // each turn, add node to result, then look for any other nodes that have their dependencies now
    // satisfied to be visited in the next loop iteration.
    while let Some(a) = pending_nodes.pop() {
        result.push(a.clone());

        for b in nodes {
            let deps = pending_deps.get_mut(b).expect("node exists");
            if deps.remove(a) && deps.is_empty() {
                pending_nodes.push(b);
            }
        }
    }

    // If any uncleared edges remain, their exists a loop and no topological sort exists.
    if pending_deps.values().any(|deps| !deps.is_empty()) {
        return None;
    }

    Some(result)
}

fn is_well_known_data_type(data_type_id: &ua::NodeId) -> bool {
    // TODO: Add proper support for Simatic data types.
    data_type_id.is_ns0() || (data_type_id.namespace_index() == 3 && data_type_id.is_numeric())
}
