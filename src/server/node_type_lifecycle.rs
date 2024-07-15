use std::ffi::c_void;

use crate::ua;
use crate::ua::NodeId;
use crate::Server;

use std::panic::AssertUnwindSafe;

use open62541_sys::UA_NodeId;
use open62541_sys::UA_NodeTypeLifecycle;
use open62541_sys::UA_Server;
use open62541_sys::UA_StatusCode;

use crate::server::NodeContext;
use crate::DataType;
use crate::ServerBuilder;

pub trait Lifecycle {
    fn constructor(
        &mut self,
        server: &mut Server,
        session_id: &NodeId,
        session_context: *mut c_void,
        type_id: &NodeId,
        type_context: *mut c_void,
        node_id: &NodeId,
    ) -> ua::StatusCode;

    fn destructor(
        &mut self,
        server: &mut Server,
        session_id: &NodeId,
        session_context: *mut c_void,
        type_id: &NodeId,
        type_context: *mut c_void,
        node_id: &NodeId,
    );
}

pub struct NodeTypeLifecycle {}

impl NodeTypeLifecycle {
    pub fn wrap_lifecycle(
        lifecycle: (impl Lifecycle + 'static),
    ) -> (UA_NodeTypeLifecycle, NodeContext) {
        unsafe extern "C" fn constructor_c(
            server: *mut UA_Server,
            session_id: *const UA_NodeId,
            session_context: *mut c_void,
            type_node_id: *const UA_NodeId,
            type_node_context: *mut c_void,
            node_id: *const UA_NodeId,
            node_context: *mut *mut c_void,
        ) -> UA_StatusCode {
            let node_context = unsafe { NodeContext::peek_at(*node_context) };
            #[allow(irrefutable_let_patterns)] // We will add more node context types eventually.
            let NodeContext::Lifecycle(lifecycle) = node_context
            else {
                // We expect to always find this node context type.
                return ua::StatusCode::BADINTERNALERROR.into_raw();
            };

            let mut lifecycle = AssertUnwindSafe(lifecycle);

            let mut server =
                unsafe { ServerBuilder::from_raw_server(server.as_mut().unwrap_unchecked()) };

            let status_code = unsafe {
                lifecycle.constructor(
                    &mut server,
                    &NodeId::clone_raw(session_id.as_ref().expect("Callback failed!")),
                    session_context,
                    &NodeId::clone_raw(type_node_id.as_ref().expect("Callback failed!")),
                    type_node_context,
                    &NodeId::clone_raw(node_id.as_ref().expect("Callback failed!")),
                )
            };

            // Forget the server so we don't call the destructor on it
            // We only constructed it so we have access to it, the server
            // should still be valid after this callback.
            std::mem::forget(server);

            status_code.into_raw()
        }

        unsafe extern "C" fn destructor_c(
            server: *mut UA_Server,
            session_id: *const UA_NodeId,
            session_context: *mut c_void,
            type_node_id: *const UA_NodeId,
            type_node_context: *mut c_void,
            node_id: *const UA_NodeId,
            node_context: *mut *mut c_void,
        ) {
            let node_context = unsafe { NodeContext::peek_at(*node_context) };
            #[allow(irrefutable_let_patterns)] // We will add more node context types eventually.
            let NodeContext::Lifecycle(lifecycle) = node_context
            else {
                panic!("Destructor failed!");
            };

            let mut lifecycle = AssertUnwindSafe(lifecycle);

            unsafe {
                lifecycle.destructor(
                    &mut ServerBuilder::from_raw_server(server.as_mut().unwrap_unchecked()),
                    &NodeId::clone_raw(session_id.as_ref().expect("Callback failed!")),
                    session_context,
                    &NodeId::clone_raw(type_node_id.as_ref().expect("Callback failed!")),
                    type_node_context,
                    &NodeId::clone_raw(node_id.as_ref().expect("Callback failed!")),
                );
            };
        }

        let raw_node_type_lifecycle = UA_NodeTypeLifecycle {
            constructor: Some(constructor_c),
            destructor: Some(destructor_c),
        };

        let node_context = NodeContext::Lifecycle(Box::new(lifecycle));

        (raw_node_type_lifecycle, node_context)
    }

    // fn new(lifecycle: &impl Lifecycle) -> Self {

    //     // Self(UA_NodeTypeLifecycle {
    //     //     constructor:,
    //     //     destructor:,
    //     // })
    // }
}
