use std::ffi::c_void;

use crate::{server::DataSource, Userdata};

/// Context attached to server node.
///
/// Nodes created by [`Server`](crate::Server) need to keep track of dynamic data structures. These
/// are cleaned up when the corresponding node is destroyed by the server.
pub enum NodeContext {
    DataSource(Box<dyn DataSource>),
}

#[allow(dead_code)] // We will use the methods soon.
impl NodeContext {
    /// Leaks node context.
    ///
    /// This allocates memory. To prevent memory leaks, make sure to call [`consume()`] on the
    /// returned pointer exactly once.
    ///
    /// [`consume()`]: Self::consume
    pub(crate) fn leak(self) -> *mut c_void {
        Userdata::<Self>::prepare(self)
    }

    /// Unwraps [`c_void`] pointer to access node context.
    ///
    /// # Safety
    ///
    /// The given pointer must have been returned from [`leak()`]. It must not have been given to
    /// [`consume()`] yet.
    ///
    /// The lifetime of the returned reference is not allowed to extend past the next call to either
    /// [`peek_at()`] or [`consume()`].
    ///
    /// [`leak()`]: Self::leak
    /// [`peek_at()`]: Self::peek_at
    /// [`consume()`]: Self::consume
    pub(crate) unsafe fn peek_at<'a>(data: *mut c_void) -> &'a mut Self {
        // SAFETY: We require the same safety guarantees from our callers.
        unsafe { Userdata::<Self>::peek_at(data) }
    }

    /// Unwraps [`c_void`] pointer and returns owned node context.
    ///
    /// # Safety
    ///
    /// The given pointer must have been returned from [`leak()`]. It must not have been given to
    /// [`consume()`] yet.
    ///
    /// [`leak()`]: Self::leak
    /// [`consume()`]: Self::consume
    #[must_use]
    pub(crate) unsafe fn consume(data: *mut c_void) -> Self {
        // SAFETY: We require the same safety guarantees from our callers.
        unsafe { Userdata::<Self>::consume(data) }
    }
}
