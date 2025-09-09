use std::{ffi::c_void, marker::PhantomData};

use crate::Userdata;

/// Type-erased callback for a mutable closure.
///
/// Use this to wrap an [`FnMut`] closure into a data structure that may be passed via a [`c_void`]
/// pointer as user data to an external library. Later, when this `extern` callback is run with that
/// data, we may unwrap it and can thus call our initial closure.
///
/// The implementation uses associated methods:
///
/// - [`CallbackMut::prepare()`] to wrap the closure and get the [`c_void`] pointer
/// - [`CallbackMut::execute()`] to unwrap the [`c_void`] pointer and call the closure repeatedly
/// - [`CallbackMut::delete()`] to drop the closure and free all memory
#[derive(Debug)]
pub(crate) struct CallbackMut<T: 'static>(PhantomData<T>);

// TODO: Use inherent associated type to define this directly on `CallbackOnce`. At the moment, this
// is not possible yet.
// https://github.com/rust-lang/rust/issues/8995
type CallbackMutUserdata<T> = Userdata<Box<dyn FnMut(T) + 'static>>;

impl<T> CallbackMut<T> {
    /// Prepares closure for later call.
    ///
    /// This allocates memory. To prevent memory leaks, call [`execute()`](CallbackMut::delete) on
    /// the returned pointer exactly once.
    pub(crate) fn prepare<F>(f: F) -> *mut c_void
    where
        F: FnMut(T) + 'static,
    {
        CallbackMutUserdata::<T>::prepare(Box::new(f))
    }

    /// Unwraps [`c_void`] pointer and calls closure.
    ///
    /// # Safety
    ///
    /// The given pointer must have been returned from [`prepare()`](CallbackMut::prepare) and must
    /// not have been passed into [`delete()`](CallbackMut::delete) yet.
    ///
    /// The value type `T` must be the same as in [`prepare()`](CallbackMut::prepare).
    pub(crate) unsafe fn execute(data: *mut c_void, payload: T) {
        let f = unsafe { CallbackMutUserdata::<T>::peek_at(data) };
        f(payload);
    }

    /// Unwraps [`c_void`] pointer and frees the memory.
    ///
    /// # Safety
    ///
    /// The given pointer must have been returned from [`prepare()`](CallbackMut::prepare).
    pub(crate) unsafe fn delete(data: *mut c_void) {
        let f = unsafe { CallbackMutUserdata::<T>::consume(data) };
        drop(f);
    }
}
