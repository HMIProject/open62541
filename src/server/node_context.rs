use std::{
    ffi::c_void,
    sync::{Arc, Mutex},
};

use crate::{server::DataSource, Lifecycle, Userdata};

/// Context attached to server node.
///
/// Nodes created by [`Server`](crate::Server) need to keep track of dynamic data structures. These
/// are cleaned up when the corresponding node is destroyed by the server.
#[derive(Clone)]
pub enum NodeContext {
    // ARC is used here to be able to reference the same `NodeContext` multiple times.
    // This is especially useful for `Lifecycle`, as the same constructor and destructor and therefore
    // `NodeContext` will be used by multiple objects.
    // Using ARC enables easy cloning of the `NodeContext`, and no shady code needs to be involved.
    // As DataSource and Lifecycle both use static lifetime, there is no need to worry about the
    // reference count and memory deallocation.
    // Using Box<> would have been sufficient for DataSource, but using Arc<> brings no relevant
    // disadvantage and has the big advantage of enabling to use `#derive[Clone]]`, which is required
    // for `Lifecycle`
    // In theory, it would be suffienct to not clone the NodeContext directly, but to use the `c_void`
    // pointer multiple times. But then handling such pointers in other parts of the code would have
    // been required, which isn't exactly nice to do.
    // Mutex is required for `DataSource` as fields in a struct implementing the trait may be mutated,
    // which could make problems with multi-threading.
    DataSource(Arc<Mutex<dyn DataSource + 'static + Send + Sync>>),
    Lifecycle(Arc<dyn Lifecycle + 'static + Send + Sync>),
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
