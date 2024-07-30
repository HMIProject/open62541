use std::ffi::c_void;

use crate::ua;
use crate::ua::NodeId;
use crate::Server;

/// Holds constructor and destructor for a `Node` type
///
/// Implement this trait on a struct to be able to use the
/// `LifecycleManager`.
pub trait Lifecycle {
    /// Constructor for a node where this `Lifecycle` was added to.
    ///
    /// Only returns `ua::StatusCode::GOOD` and does
    /// nothing else by default.
    ///
    /// Has a shared reference to self, so there is no need to worry
    /// about thread-safety, if a struct implementing this trait
    /// has fields.
    #[allow(unused_variables)]
    fn constructor(
        &self,
        session_id: &NodeId,
        session_context: *mut c_void,
        type_id: &NodeId,
        type_context: *mut c_void,
        node_id: &NodeId,
    ) -> ua::StatusCode {
        ua::StatusCode::GOOD
    }

    /// Destructor for a node where this `Lifecycle` was added to.
    ///
    /// Does nothing by default.
    ///
    /// Has a shared reference to self, so there is no need to worry
    /// about thread-safety, if a struct implementing this trait
    /// has fields.
    #[allow(unused_variables)]
    fn destructor(
        &self,
        session_id: &NodeId,
        session_context: *mut c_void,
        type_id: &NodeId,
        type_context: *mut c_void,
        node_id: &NodeId,
    ) {
    }
}
