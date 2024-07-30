use crate::ua::{self, NodeTypeLifecycle};

use crate::{server::Node, Attributes, NodeContext};

use crate::{Error, Lifecycle, Result, Server};
use std::sync::Arc;

use open62541_sys::UA_NodeId;
use open62541_sys::UA_Server;
use open62541_sys::UA_StatusCode;

use crate::DataType;
use crate::ServerBuilder;

use std::ffi::c_void;

/// Wrapper for handling Node lifecycle
///
/// If you have created your own `VariableType` or `ObjectType`
/// this may be the perfect fit for you. Using this wrapper,
/// you can add a constructor and destructor to this type.
/// These will be called when a Variable node or an Object node
/// with your corresponding type gets created.
/// To make use of this functionality view Usage below.
///
/// # Usage
///
/// * First call `LifecycleManager::init()` to crate an object.
/// * Then use `LifecycleManager::with_type_node()` and
///   `LifecycleManager::with_lifecycle()` to initialize the
///   fields so you can call the following two methods.
/// * To register your `Lifecycle` implementation on the server,
///   run `LifecycleManager::server_set_node_type_lifecylce()`.
/// * When you are then trying to add a new node with your
///   `Node` type (`VariableType` or `ObjectType`) as type,
///   make sure to use
///   `LifecycleManager::server_add_node_with_lifecylce()`
///   as the information for your constructor/destructor
///   callbacks needs to be added to the node.
/// * If you have used `Server::addNode()` instead by accident,
///   the `Lifecycle` callbacks will not be called. And you'll
///   be left with an error.
///
#[allow(clippy::struct_field_names)]
pub struct LifecycleManager<'a> {
    node_type_id: Option<&'a ua::NodeId>,
    node_context: Option<NodeContext>,
    node_type_lifecycle: Option<ua::NodeTypeLifecycle>,
}

impl<'a> LifecycleManager<'a> {
    #[must_use]
    pub const fn init() -> Self {
        Self {
            node_type_id: None,
            node_context: None,
            node_type_lifecycle: None,
        }
    }

    /// Adds a `Node` with information on where to find the constructor and destructor
    ///
    /// # Errors
    ///
    /// This errors when `LifecycleManager::with_lifecycle()` has not yet been called
    /// successfully or the `Node` could not be added.
    pub fn server_add_node_with_lifecylce<T: Attributes>(
        &self,
        server: &Server,
        mut node: Node<T>,
    ) -> Result<()> {
        // ARC count to the lifecycle trait impl increases by one here. View the
        // definition of `NodeContext` for more information.
        let node_context = self.node_context.clone().ok_or(Error::internal(
            "node_context is None. It mustn't be to add the node!
            Consider calling `LifecycleManager::with_lifecycle()`!",
        ))?;
        node.context = Some(node_context);
        server.add_node(node)?;
        Ok(())
    }

    /// Adds `Lifecycle` constructor / deconstructor to the `Node` type
    ///
    /// # Errors
    ///
    /// This errors either when `LifecycleManager::with_type_node()` and
    /// `LifecycleManager::with_lifecycle()` have not yet been called
    /// successfully or this function was called a second time.
    pub fn server_set_node_type_lifecylce(mut self, server: &Server) -> Result<Self> {
        let type_node_id: &ua::NodeId = self.node_type_id.ok_or(Error::internal(
            "type_node_id is None. It mustn't be to set the lifecycle!
            Consider calling `LifecycleManager::with_type_node()`!",
        ))?;
        let node_type_lifecycle: ua::NodeTypeLifecycle =
            self.node_type_lifecycle.ok_or(Error::internal(
                "node_type_li is None. It mustn't be to set the lifecycle!
            Consider calling `LifecycleManager::with_lifecycle()`!",
            ))?;
        server.set_node_type_lifecycle(type_node_id, node_type_lifecycle);
        self.node_type_lifecycle = None;
        Ok(self)
    }

    /// Specify the type `NodeId` which will be used with this `LifecycleManager` object.
    /// Returns the object which has the `NodeId` set.
    #[must_use]
    pub const fn with_type_node(mut self, node_type_id: &'a ua::NodeId) -> Self {
        self.node_type_id = Some(node_type_id);
        self
    }

    /// Specify the `Lifecycle` trait implementation which will be used with this
    /// `LifecycleManager` object.
    /// Returns the object which has the `Lifecycle` trait implementation set.
    /// Internally this sets up the `NodeContext` with the extern C functions that
    /// call the functions on the given `Lifecycle` trait implementation.
    #[must_use]
    pub fn with_lifecycle(
        mut self,
        lifecycle: (impl Lifecycle + 'static + std::marker::Send + std::marker::Sync),
    ) -> Self {
        unsafe extern "C" fn constructor_c(
            _server: *mut UA_Server,
            session_id: *const UA_NodeId,
            session_context: *mut c_void,
            type_node_id: *const UA_NodeId,
            type_node_context: *mut c_void,
            node_id: *const UA_NodeId,
            node_context: *mut *mut c_void,
        ) -> UA_StatusCode {
            let node_context = unsafe { NodeContext::peek_at(*node_context) };
            let NodeContext::Lifecycle(lifecycle) = node_context else {
                log::error!("Could not convert from *mut *mut c_void to Rust `NodeContext::Lifecycle`! \
                Consider using `LifecycleManager::server_add_node_with_lifecylce()` to add the node to the server. \
                This will make sure the correct NodeContext is passed. \
                There is also a chance that some memory problem occurred.");
                // We expect to always find this node context type.
                return ua::StatusCode::BADINTERNALERROR.into_raw();
            };

            // We expect the passed parameters to be valid. If they aren't, something went wrong badly and we
            // panic here.
            let panic_str = "Invalid parameters passed to the callback. Callback failed!";

            let status_code = unsafe {
                lifecycle.constructor(
                    &ua::NodeId::clone_raw(session_id.as_ref().expect(panic_str)),
                    session_context,
                    &ua::NodeId::clone_raw(type_node_id.as_ref().expect(panic_str)),
                    type_node_context,
                    &ua::NodeId::clone_raw(node_id.as_ref().expect(panic_str)),
                )
            };

            status_code.into_raw()
        }

        unsafe extern "C" fn destructor_c(
            _server: *mut UA_Server,
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
                log::error!(
                    "Could not convert from *mut *mut c_void to Rust `NodeContext::Lifecycle`! \
                    If the constructor ran successfully: Maybe the memory where `NodeContext::Lifecycle` \
                    was stored is invalid now?"
                );
                panic!("Destructor failed! View the log for more details!");
            };

            // We expect the passed parameters to be valid. If they aren't, something went wrong badly and we
            // panic here.
            let panic_str = "Invalid parameters passed to the callback. Callback failed!";

            unsafe {
                lifecycle.destructor(
                    &ua::NodeId::clone_raw(session_id.as_ref().expect(panic_str)),
                    session_context,
                    &ua::NodeId::clone_raw(type_node_id.as_ref().expect(panic_str)),
                    type_node_context,
                    &ua::NodeId::clone_raw(node_id.as_ref().expect(panic_str)),
                );
            };

            // The ARC to node_context could be decreased here by 1 but is not
            // necessary and could introduce unsafety. It isn't necessary as the
            // memory for the rust con/destructor has static lifetime.
        }

        self.node_type_lifecycle = Some(
            NodeTypeLifecycle::init()
                .with_constructor(constructor_c)
                .with_destructor(destructor_c),
        );

        // ARC to lifecycle trait impl is created here. View the
        // definition of `NodeContext` for more information.
        self.node_context = Some(NodeContext::Lifecycle(Arc::new(lifecycle)));

        self
    }
}
