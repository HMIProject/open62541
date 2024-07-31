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
/// The returned [`UA_DataSource`] is only valid for as long as [`NodeContext`] is alive. The
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
        // #[allow(irrefutable_let_patterns)] // We will add more node context types eventually.
        let NodeContext::MethodCallback(callback) = node_context else {
            // We expect to always find this node context type.
            return ua::StatusCode::BADINTERNALERROR.into_raw();
        };

        let callback = AssertUnwindSafe(callback);

        let session_id = ua::NodeId::clone_raw(unsafe { session_id.as_ref().expect("todo") });
        let method_id = ua::NodeId::clone_raw(unsafe { method_id.as_ref().expect("todo") });
        let object_id = ua::NodeId::clone_raw(unsafe { object_id.as_ref().expect("todo") });
        let input = ua::Array::<ua::Variant>::from_raw_parts(input_size, input).expect("todo");

        let status_code = match catch_unwind(move || {
            callback.callback(session_id, method_id, object_id, input)
        }) {
            Ok(Ok(return_value)) => {
                // Check if we can safely move the output array into the c array pointer
                assert!(output_size == return_value.len());

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

/*pub type MethodCallback = fn(
    server: *mut UA_Server,
    sessionId: *const UA_NodeId,
    sessionContext: *mut ::core::ffi::c_void,
    methodId: *const UA_NodeId,
    methodContext: *mut ::core::ffi::c_void,
    objectId: *const UA_NodeId,
    objectContext: *mut ::core::ffi::c_void,
    inputSize: usize,
    input: *const UA_Variant,
    outputSize: usize,
    output: *mut UA_Variant,
) -> ua::StatusCode;
*/
