use core::ffi::c_void;
use open62541_sys::{UA_NodeId, UA_Server, UA_StatusCode};

// Does not work as there is no UA_TYPES_NODETYPELIFECYCLE (bug in open62541?)
// crate::data_type!(NodeTypeLifecycle);

// For now let's use the expanded macro:
/// Wrapper for
///[`UA_NodeTypeLifecycle`](open62541_sys::UA_NodeTypeLifecycle)
/// from [`open62541_sys`].
///
/// This owns the wrapped data. When the wrapper is dropped, the inner value is cleaned up
/// with [`UA_clear()`] to release dynamically allocated memory held by the value.
///
/// [`UA_clear()`]: open62541_sys::UA_clear
#[repr(transparent)]
pub struct NodeTypeLifecycle(
    /// Inner value.
    open62541_sys::UA_NodeTypeLifecycle,
);
unsafe impl Send for NodeTypeLifecycle {}
unsafe impl Sync for NodeTypeLifecycle {}
impl Drop for NodeTypeLifecycle {
    fn drop(&mut self) {
        unsafe {
            open62541_sys::UA_clear(
                std::ptr::addr_of_mut!(self.0).cast::<std::ffi::c_void>(),
                <Self as crate::DataType>::data_type(),
            );
        }
    }
}
unsafe impl crate::DataType for NodeTypeLifecycle {
    type Inner = open62541_sys::UA_NodeTypeLifecycle;
    fn data_type() -> *const open62541_sys::UA_DataType {
        // let index = usize::try_from(open62541_sys::UA_TYPES_NODETYPELIFECYCLE).unwrap();
        // unsafe { open62541_sys::UA_TYPES.get(index) }.unwrap()
        panic!(
            "Cannot call data_type() with NodeTypeLifecycle right now as
            UA_TYPES_NODETYPELIFECYCLE  does not exist in open62541."
        )
    }
    #[must_use]
    unsafe fn from_raw(src: Self::Inner) -> Self {
        NodeTypeLifecycle(src)
    }
    #[must_use]
    fn into_raw(self) -> Self::Inner {
        let inner = unsafe { std::ptr::read(std::ptr::addr_of!(self.0)) };
        std::mem::forget(self);
        inner
    }
}
impl Clone for NodeTypeLifecycle {
    fn clone(&self) -> Self {
        <Self as crate::DataType>::clone_raw(&self.0)
    }
}
impl std::fmt::Debug for NodeTypeLifecycle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = <Self as crate::DataType>::print(self);
        let string = output.as_ref().and_then(|output| output.as_str());
        f.write_str(string.unwrap_or("NodeTypeLifecycle"))
    }
}
impl std::cmp::PartialEq for NodeTypeLifecycle {
    fn eq(&self, other: &Self) -> bool {
        <Self as std::cmp::Ord>::cmp(self, other) == std::cmp::Ordering::Equal
    }
}
impl std::cmp::Eq for NodeTypeLifecycle {}
impl std::cmp::PartialOrd for NodeTypeLifecycle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(<Self as std::cmp::Ord>::cmp(self, other))
    }
}
impl std::cmp::Ord for NodeTypeLifecycle {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let result = <Self as crate::DataType>::order(self, other);
        match result {
            open62541_sys::UA_Order::UA_ORDER_LESS => std::cmp::Ordering::Less,
            open62541_sys::UA_Order::UA_ORDER_EQ => std::cmp::Ordering::Equal,
            open62541_sys::UA_Order::UA_ORDER_MORE => std::cmp::Ordering::Greater,
            _ => {
                panic!("should return valid order");
            }
        }
    }
}

pub type ConstructorFn = unsafe extern "C" fn(
    server: *mut UA_Server,
    session_id: *const UA_NodeId,
    session_context: *mut c_void,
    type_node_id: *const UA_NodeId,
    type_node_context: *mut c_void,
    node_id: *const UA_NodeId,
    node_context: *mut *mut c_void,
) -> UA_StatusCode;

pub type DestructorFn = unsafe extern "C" fn(
    server: *mut UA_Server,
    session_id: *const UA_NodeId,
    session_context: *mut c_void,
    type_node_id: *const UA_NodeId,
    type_node_context: *mut c_void,
    node_id: *const UA_NodeId,
    node_context: *mut *mut c_void,
);

impl NodeTypeLifecycle {
    #[must_use]
    pub fn with_constructor(mut self, constructor: ConstructorFn) -> Self {
        self.0.constructor = Some(constructor);
        self
    }

    #[must_use]
    pub fn with_destructor(mut self, destructor: DestructorFn) -> Self {
        self.0.destructor = Some(destructor);
        self
    }
}
