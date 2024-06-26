use std::{ffi::c_void, marker::PhantomData};

use tokio::sync::mpsc;

use crate::Userdata;

/// Type-erased one-shot callback.
///
/// Use this to wrap an [`FnOnce`] closure into a data structure that may be passed via a [`c_void`]
/// pointer as user data to an external library. Later, when this `extern` callback is run with that
/// data, we may unwrap it and can thus call our initial closure.
///
/// The implementation uses associated methods:
///
/// - [`CallbackOnce::prepare()`] to wrap the closure and get the [`c_void`] pointer
/// - [`CallbackOnce::execute()`] to unwrap the [`c_void`] pointer and call the closure
///
/// # Examples
///
/// ```
/// use open62541::CallbackOnce;
/// use std::{cell::Cell, rc::Rc};
/// # use std::ffi::c_void;
///
/// let cell = Rc::new(Cell::new(0));
///
/// // Turn `tx` into type-erased void pointer for FFI.
/// let raw_data: *mut c_void = CallbackOnce::<u32>::prepare({
///     let cell = Rc::clone(&cell);
///     move |value| {
///         cell.set(value);
///     }
/// });
///
/// // Use type-erased pointer to call closure.
/// unsafe { CallbackOnce::<u32>::execute(raw_data, 123); }
///
/// // Value has been received.
/// assert_eq!(cell.get(), 123);
/// ```
#[allow(clippy::module_name_repetitions)]
pub struct CallbackOnce<T>(PhantomData<T>);

// TODO: Use inherent associated type to define this directly on `CallbackOnce`. At the moment, this
// is not possible yet.
// https://github.com/rust-lang/rust/issues/8995
type CallbackOnceUserdata<T> = Userdata<Box<dyn FnOnce(T) + 'static>>;

impl<T> CallbackOnce<T> {
    /// Prepares closure for later call.
    ///
    /// This allocates memory. To prevent memory leaks, call [`execute()`](CallbackOnce::execute) on
    /// the returned pointer exactly once.
    pub fn prepare<F>(f: F) -> *mut c_void
    where
        F: FnOnce(T) + 'static,
    {
        CallbackOnceUserdata::<T>::prepare(Box::new(f))
    }

    /// Unwraps [`c_void`] pointer and calls closure.
    ///
    /// # Safety
    ///
    /// The given pointer must have been returned from [`prepare()`](CallbackOnce::prepare) and must
    /// not have been passed into [`execute()`](CallbackOnce::execute) yet.
    ///
    /// The value type `T` must be the same as in [`prepare()`](CallbackOnce::prepare).
    pub unsafe fn execute(data: *mut c_void, payload: T) {
        let f = unsafe { CallbackOnceUserdata::<T>::consume(data) };
        f(payload);
    }
}

/// Type-erased stream sender.
///
/// Use this to wrap a [`Sender`] into a data structure that may be passed via a [`c_void`] pointer
/// as user data to an external library. Later, when this `extern` callback is run with that data,
/// we may unwrap it and can thus send messages into the stream or close the stream.
///
/// The implementation uses associated methods:
///
/// - [`CallbackStream::prepare()`] to wrap the stream sender and get the [`c_void`] pointer
/// - [`CallbackStream::notify()`] to use the [`c_void`] pointer and send messages to the stream
/// - [`CallbackStream::delete()`] to unwrap the [`c_void`] pointer and close the underlying stream
///
/// [`Sender`]: mpsc::Sender
///
/// # Examples
///
/// ```
/// use futures::executor::block_on;
/// use open62541::CallbackStream;
/// # use std::ffi::c_void;
/// use tokio::sync::mpsc;
///
/// let (tx, mut rx) = mpsc::channel::<u32>(10);
///
/// // Turn `tx` into type-erased void pointer for FFI.
/// let raw_data: *mut c_void = CallbackStream::<u32>::prepare(tx);
///
/// // Use type-erased pointer to send messages.
/// unsafe { CallbackStream::<u32>::notify(raw_data, 1); }
/// unsafe { CallbackStream::<u32>::notify(raw_data, 2); }
/// unsafe { CallbackStream::<u32>::notify(raw_data, 3); }
///
/// // Clean up resources held by pointer.
/// unsafe { CallbackStream::<u32>::delete(raw_data); }
///
/// // Values have been received.
/// assert_eq!(block_on(rx.recv()), Some(1));
/// assert_eq!(block_on(rx.recv()), Some(2));
/// assert_eq!(block_on(rx.recv()), Some(3));
/// assert_eq!(block_on(rx.recv()), None);
/// ```
#[allow(clippy::module_name_repetitions)]
pub struct CallbackStream<T>(PhantomData<T>);

// TODO: Use inherent associated type to define this directly on `CallbackOnce`. At the moment, this
// is not possible yet.
// https://github.com/rust-lang/rust/issues/8995
type CallbackStreamUserdata<T> = Userdata<mpsc::Sender<T>>;

impl<T> CallbackStream<T> {
    /// Prepares sender for later use.
    ///
    /// This allocates memory. To prevent memory leaks, call [`delete()`](CallbackStream::delete) on
    /// the returned pointer exactly once.
    #[must_use]
    pub fn prepare(tx: mpsc::Sender<T>) -> *mut c_void {
        CallbackStreamUserdata::<T>::prepare(tx)
    }

    /// Uses [`c_void`] pointer and sends message.
    ///
    /// # Safety
    ///
    /// The given pointer must have been returned from [`prepare()`](CallbackStream::prepare) and
    /// must not have been passed into [`delete()`](CallbackStream::delete) yet.
    ///
    /// The value type `T` must be the same as in [`prepare()`](CallbackStream::prepare).
    pub unsafe fn notify(data: *mut c_void, payload: T) {
        let tx = unsafe { CallbackStreamUserdata::<T>::peek_at(data) };
        // Send message. Ignore disconnects and full buffers. (There is not much we can do here when
        // the buffer is full. We could blockingly wait but that blocks `UA_Client_run_iterate()` in
        // our event loop, potentially preventing the receiver from clearing the stream.)
        let _unused = tx.try_send(payload);
    }

    /// Unwraps [`c_void`] pointer and closes channel.
    ///
    /// # Safety
    ///
    /// The given pointer must have been returned from [`prepare()`](CallbackStream::prepare) and
    /// must not have been passed into [`delete()`](CallbackStream::delete) yet.
    pub unsafe fn delete(data: *mut c_void) {
        let _unused = unsafe { CallbackStreamUserdata::<T>::consume(data) };
    }
}
