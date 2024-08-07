use ::core::ffi::c_void;
use std::{
    panic::{catch_unwind, AssertUnwindSafe},
    ptr::NonNull,
};

use open62541_sys::{
    UA_MethodCallback, UA_NodeId, UA_Server, UA_StatusCode, UA_Variant, UA_EMPTY_ARRAY_SENTINEL,
};
use thiserror::Error;

use crate::{server::NodeContext, ua, DataType as _};

/// Result from [`MethodCallback`] operations.
///
/// On success, the operations return `Ok(())`. The actual value is transmitted through the
/// `context` argument. See [`MethodCallback::call()`] for details.
#[allow(clippy::module_name_repetitions)]
pub type MethodCallbackResult = Result<(), MethodCallbackError>;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Error)]
pub enum MethodCallbackError {
    #[error("{0}")]
    StatusCode(ua::StatusCode),

    #[error("not supported")]
    NotSupported,
}

impl MethodCallbackError {
    pub(crate) fn into_status_code(self) -> ua::StatusCode {
        match self {
            MethodCallbackError::StatusCode(status_code) => status_code,
            MethodCallbackError::NotSupported => ua::StatusCode::BADNOTSUPPORTED,
        }
    }
}

impl From<ua::StatusCode> for MethodCallbackError {
    fn from(value: ua::StatusCode) -> Self {
        // Any good error would be misleading.
        Self::StatusCode(if value.is_good() {
            ua::StatusCode::BADINTERNALERROR
        } else {
            value
        })
    }
}

/// Method callback.
///
/// The `call` callback implement the operation on the method when it is added via
/// [`Server::add_method_node()`].
///
/// [`Server::add_method_node()`]: crate::Server::add_method_node
pub trait MethodCallback {
    /// Calls method.
    ///
    /// This is called when a client wants to call the method. The input arguments are available,
    /// and the output arguments are expected to be returned, through the `context` argument. See
    /// [`MethodCallbackContext::input_arguments()`] and
    /// [`MethodCallbackContext::set_output_arguments()`] for details.
    ///
    /// # Errors
    ///
    /// This should return an appropriate error when the call is not possible. The underlying status
    /// code is forwarded to the client.
    // TODO: Check if we can guarantee `&mut self`.
    fn call(&mut self, context: &mut MethodCallbackContext) -> MethodCallbackResult;
}

/// Context when [`DataSource`] is being read from.
#[allow(clippy::module_name_repetitions)]
pub struct MethodCallbackContext {
    object_id: NonNull<UA_NodeId>,
    input_size: usize,
    input_source: NonNull<UA_Variant>,
    output_size: usize,
    output_target: NonNull<UA_Variant>,
}

impl MethodCallbackContext {
    /// Creates context for `call` callback.
    fn new(
        object_id: *const UA_NodeId,
        input_size: usize,
        input: *const UA_Variant,
        output_size: usize,
        output: *mut UA_Variant,
    ) -> Option<Self> {
        let ptr = unsafe { UA_EMPTY_ARRAY_SENTINEL };

        // We do not expect the empty array sentinel here.
        if input == ptr.cast::<UA_Variant>() {
            return None;
        }
        if output == ptr.cast::<UA_Variant>().cast_mut() {
            return None;
        }

        Some(Self {
            // SAFETY: `NonNull` implicitly expects a `*mut` but we take care to never mutate the
            // target.
            object_id: NonNull::new(object_id.cast_mut())?,
            input_size,
            // SAFETY: `NonNull` implicitly expects a `*mut` but we take care to never mutate the
            // target.
            input_source: NonNull::new(input.cast_mut())?,
            output_size,
            output_target: NonNull::new(output)?,
        })
    }

    /// Gets object node ID.
    ///
    /// This returns the object node ID used by the client that is calling this [`MethodCallback`].
    pub fn object_id(&self) -> &ua::NodeId {
        let object_id = unsafe { self.object_id.as_ref() };
        ua::NodeId::raw_ref(object_id)
    }

    /// Gets input arguments.
    ///
    /// This returns the values received from the client that is calling this [`MethodCallback`].
    #[must_use]
    pub fn input_arguments(&self) -> &[ua::Variant] {
        let Some(input_arguments) = (unsafe {
            ua::Array::slice_from_raw_parts(self.input_size, self.input_source.as_ptr())
        }) else {
            // PANIC: We should never receive an invalid array (as defined by OPC UA).
            unreachable!("received invalid input arguments array");
        };

        input_arguments
    }

    /// Sets output arguments.
    ///
    /// This sets the values to report back to the client that is calling this [`MethodCallback`].
    // TODO: Make error more specific.
    pub fn set_output_arguments(&mut self, values: &[ua::Variant]) -> Result<(), ()> {
        let Some(output_arguments) = (unsafe {
            ua::Array::slice_from_raw_parts_mut(self.output_size, self.output_target.as_ptr())
        }) else {
            // PANIC: We should never receive an invalid array (as defined by OPC UA).
            unreachable!("received invalid input arguments array");
        };

        if values.len() != output_arguments.len() {
            return Err(());
        }

        for (output_argument, value) in output_arguments.iter_mut().zip(values.iter()) {
            value.clone_into(output_argument);
        }

        Ok(())
    }
}

/// Transforms into raw value.
///
/// # Safety
///
/// The returned [`UA_MethodCallback`] is only valid for as long as [`NodeContext`] is alive. The
/// lifetime can be extended by using [`NodeContext::leak()`] to save this value inside the
/// corresponding server node, to be eventually cleaned up when the node is destroyed.
pub(crate) unsafe fn wrap_method_callback(
    method_callback: impl MethodCallback + 'static,
) -> (UA_MethodCallback, NodeContext) {
    unsafe extern "C" fn callback_c(
        _server: *mut UA_Server,
        _session_id: *const UA_NodeId,
        _session_context: *mut c_void,
        _method_id: *const UA_NodeId,
        method_context: *mut c_void,
        object_id: *const UA_NodeId,
        _object_context: *mut c_void,
        input_size: usize,
        input: *const UA_Variant,
        output_size: usize,
        output: *mut UA_Variant,
    ) -> UA_StatusCode {
        let node_context = unsafe { NodeContext::peek_at(method_context) };
        #[allow(irrefutable_let_patterns)] // We will add more node context types eventually.
        let NodeContext::MethodCallback(method_callback) = node_context
        else {
            // We expect to always find this node context type.
            return ua::StatusCode::BADINTERNALERROR.into_raw();
        };

        let Some(mut context) =
            MethodCallbackContext::new(object_id, input_size, input, output_size, output)
        else {
            // Creating context for callback should always succeed.
            return ua::StatusCode::BADINTERNALERROR.into_raw();
        };
        let mut method_callback = AssertUnwindSafe(method_callback);

        let status_code = match catch_unwind(move || method_callback.call(&mut context)) {
            Ok(Ok(())) => ua::StatusCode::GOOD,
            Ok(Err(err)) => err.into_status_code(),
            Err(err) => {
                log::error!("Call callback in method callback panicked: {err:?}");
                ua::StatusCode::BADINTERNALERROR
            }
        };

        status_code.into_raw()
    }

    let node_context = NodeContext::MethodCallback(Box::new(method_callback));

    (Some(callback_c), node_context)
}
