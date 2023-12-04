use std::ffi::c_void;

/// Type-removed callback that may be executed once.
///
/// Use this to wrap an [`FnOnce`] closure into a data structure that may be passed via a [`c_void`]
/// pointer as user data to an external library. Later, when this `extern` callback is run with that
/// data, we may unwrap it and can thus call our initial closure.
///
/// The implementation uses associated methods:
///
/// - [`CallbackOnce::prepare()`] to wrap the closure and get the [`c_void`] pointer
/// - [`CallbackOnce::execute()`] to unwrap the [`c_void`] pointer and call the closure
pub(crate) struct CallbackOnce<T>(Box<dyn FnOnce(T)>);

impl<T> CallbackOnce<T> {
    /// Prepares closure for later call.
    ///
    /// This allocates memory. To prevent memory leaks, call [`execute()`](CallbackOnce::execute) on
    /// the returned pointer exactly once.
    pub fn prepare<F>(f: F) -> *mut c_void
    where
        F: FnOnce(T) + 'static,
    {
        let callback = CallbackOnce(Box::new(f));
        // Move `callback` onto the heap and leak its memory into a raw pointer.
        let ptr: *mut CallbackOnce<T> = Box::into_raw(Box::new(callback));
        ptr.cast::<c_void>()
    }

    /// Unwrap [`c_void`] pointer and calls closure.
    ///
    /// # Safety
    ///
    /// The given pointer must have been returned from [`prepare()`](CallbackOnce::prepare) and must
    /// not have been passed into [`execute()`](CallbackOnce::execute) yet.
    pub unsafe fn execute(data: *mut c_void, payload: T) {
        let ptr: *mut CallbackOnce<T> = data.cast::<CallbackOnce<T>>();
        // Reconstruct heap-allocated `callback` back into its `Box`.
        let callback = *unsafe { Box::from_raw(ptr) };
        callback.0(payload);
        // Here, the `Box` is dropped and its memory freed.
    }
}
