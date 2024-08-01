use std::panic::{catch_unwind, AssertUnwindSafe};

use open62541_sys::{UA_MethodCallback, UA_NodeId, UA_Server, UA_StatusCode, UA_Variant};

use crate::{ua, DataType as _, Result};

use super::NodeContext;

pub struct MethodNodeArgumentsNodeIds {
    pub input: ua::NodeId,
    pub output: ua::NodeId,
}

impl MethodNodeArgumentsNodeIds {
    pub fn init() -> Self {
        MethodNodeArgumentsNodeIds {
            input: ua::NodeId::null(),
            output: ua::NodeId::null(),
        }
    }
}

pub trait MethodCallback {
    /// # Errors
    ///
    /// This errors when the callback was not successful
    // Contexts are not passed to the method right now. If you need something from the context,
    // create a e.g. wrapper struct around the stored data and then pass this struct to the callback.
    // Passing the raw, unsafe pointers would violate this crate's goal of providing a safe interface.
    fn callback(
        &self,
        session_id: ua::NodeId,
        // session_context: SessionContext,
        method_id: ua::NodeId,
        // method_context: MethodContext,
        object_id: ua::NodeId,
        // object_context: ObjectContext,
        input: ua::Array<ua::Variant>,
    ) -> Result<ua::Array<ua::Variant>>;
}

/// Transforms into raw value.
///
/// # Safety
///
/// The returned [`UA_MethodCallback`] is only valid for as long as [`NodeContext`] is alive. The
/// lifetime can be extended by using [`NodeContext::leak()`] to save this value inside the
/// corresponding server node, to be eventually cleaned up when the node is destroyed.
pub(crate) unsafe fn wrap_callback(
    callback: impl MethodCallback + 'static,
) -> (UA_MethodCallback, NodeContext) {
    unsafe extern "C" fn callback_c(
        _server: *mut UA_Server,
        session_id: *const UA_NodeId,
        _session_context: *mut ::core::ffi::c_void,
        method_id: *const UA_NodeId,
        method_context: *mut ::core::ffi::c_void,
        object_id: *const UA_NodeId,
        _object_context: *mut ::core::ffi::c_void,
        input_size: usize,
        input: *const UA_Variant,
        output_size: usize,
        mut output: *mut UA_Variant,
    ) -> UA_StatusCode {
        let node_context = unsafe { NodeContext::peek_at(method_context) };
        let NodeContext::MethodCallback(callback) = node_context else {
            // We expect to always find this node context type.
            return ua::StatusCode::BADINTERNALERROR.into_raw();
        };

        let callback = AssertUnwindSafe(callback);

        let panic_str = "Could not wrap around raw datatype! \
        The variable passed to this method node callback were invaild. \
        This should not happen as we expect the open62541 C server to give us pointers to valid memory. \
        This is probably a bug which should be reported!";

        let session_id = ua::NodeId::clone_raw(unsafe { session_id.as_ref().expect(&panic_str) });
        let method_id = ua::NodeId::clone_raw(unsafe { method_id.as_ref().expect(&panic_str) });
        let object_id = ua::NodeId::clone_raw(unsafe { object_id.as_ref().expect(&panic_str) });
        let input = ua::Array::<ua::Variant>::from_raw_parts(input_size, input).expect(&panic_str);

        let status_code = match catch_unwind(move || {
            callback.callback(session_id, method_id, object_id, input)
        }) {
            Ok(Ok(return_value)) => {
                // Check if we can safely move the output array into the c array pointer
                assert!(output_size == return_value.len(), "The method node callback returned an invalid amount of output arguments. \
                Make sure to return exactly the amount of arguments which were specified when adding this node to the namespace!");

                // The output size is fixed, we don't need it, but the move_into_raw requires a location to write it to.
                // To solve this, we just write into a unused variable.
                let mut size: usize = 0;
                return_value.move_into_raw(&mut size, &mut output);
                ua::StatusCode::GOOD
            }
            Ok(Err(err)) => err.status_code(),
            Err(err) => {
                log::error!("Method callback panicked: {err:?}");
                ua::StatusCode::BADINTERNALERROR
            }
        };

        status_code.into_raw()
    }

    let node_context = NodeContext::MethodCallback(Box::new(callback));

    (Some(callback_c), node_context)
}
