use std::{ffi::c_void, sync::mpsc};

/// Type-removed one-shot callback.
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
        let callback = unsafe { Box::from_raw(ptr) };
        callback.0(payload);
        // Here, the `Box` is dropped and its memory freed.
    }
}

/// Type-removed stream sender.
///
/// Use this to wrap a [`SyncSender`] into a data structure that may be passed as [`c_void`] pointer
/// as user data to an external library. Later, when this `extern` callback is called with this user
/// data, we may unwrap it and can send messages into the stream or close the stream.
///
/// The implementation uses associated methods:
///
/// - [`CallbackMut::prepare()`] to wrap the stream sender and get the [`c_void`] pointer
/// - [`CallbackMut::notify()`] to use the [`c_void`] pointer and send messages to the stream
/// - [`CallbackMut::delete()`] to unwrap the [`c_void`] pointer and close the underlying stream
///
/// [`SyncSender`]: mpsc::SyncSender
pub(crate) struct CallbackMut<T>(mpsc::SyncSender<T>);

impl<T> CallbackMut<T> {
    /// Prepares stream for later sends.
    ///
    /// This allocates memory. To prevent memory leaks, call [`delete()`](CallbackMut::delete) on
    /// the returned pointer exactly once.
    pub fn prepare(tx: mpsc::SyncSender<T>) -> *mut c_void {
        let callback = CallbackMut(tx);
        // Move `callback` onto the heap and leak its memory into a raw pointer.
        let ptr: *mut CallbackMut<T> = Box::into_raw(Box::new(callback));
        ptr.cast::<c_void>()
    }

    /// Use [`c_void`] pointer and send message.
    ///
    /// # Safety
    ///
    /// The given pointer must have been returned from [`prepare()`](CallbackMut::prepare) and must
    /// not have been passed into [`delete()`](CallbackMut::delete) yet.
    pub unsafe fn notify(data: *mut c_void, payload: T) {
        let ptr: *mut CallbackMut<T> = data.cast::<CallbackMut<T>>();
        // Reconstruct heap-allocated `callback` back into its `Box`.
        let callback = unsafe { Box::from_raw(ptr) };
        // Send message, blocking on full buffer. Ignore disconnects.
        let _unused = callback.0.send(payload);
        // Leak `callback` again to allow future notification calls.
        let _unused = Box::into_raw(callback);
    }

    /// Unwrap [`c_void`] pointer and close channel.
    ///
    /// # Safety
    ///
    /// The given pointer must have been returned from [`prepare()`](CallbackMut::prepare) and must
    /// not have been passed into [`delete()`](CallbackMut::delete) yet.
    pub unsafe fn delete(data: *mut c_void) {
        let ptr: *mut CallbackMut<T> = data.cast::<CallbackMut<T>>();
        // Reconstruct heap-allocated `callback` back into its `Box`.
        let callback = unsafe { Box::from_raw(ptr) };
        // Here, the `Box` is dropped and its memory freed.
        drop(callback);
    }
}
