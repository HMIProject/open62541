use std::{ffi::c_void, marker::PhantomData};

/// Type-erased user data.
///
/// Use this to wrap arbitrary data into a structure which may be passed via a [`c_void`] pointer to
/// an external library. Later, when this `extern` callback is run with that data, you may unwrap it
/// and can thus access the original data.
///
/// The implementation uses associated methods:
///
/// - [`Userdata::prepare()`] to wrap the user data and get a [`c_void`] pointer
/// - [`Userdata::peek_at()`] to unwrap the [`c_void`] pointer and get the user data
/// - [`Userdata::consume()`] to clean up the [`c_void`] pointer and get the user data
///
/// Each [`prepare()`] must be paired with exactly one [`consume()`] to release the data held by the
/// wrapper structure. [`peek_at()`] must only be called in-between and the lifetime of the returned
/// reference is not allowed to extend past the next call to either [`peek_at()`] or [`consume()`].
///
/// [`prepare()`]: Self::prepare
/// [`peek_at()`]: Self::peek_at
/// [`consume()`]: Self::consume
///
/// # Examples
///
/// ```
/// use open62541::Userdata;
/// # use std::ffi::c_void;
///
/// // Turn user data into type-erased void pointer for FFI.
/// let raw_data: *mut c_void = Userdata::<u32>::prepare(0);
///
/// // Use type-erased pointer to get/manipulate user data.
/// unsafe {
///     let userdata = Userdata::<u32>::peek_at(raw_data);
///     assert_eq!(*userdata, 0);
///     *userdata = 123;
/// }
///
/// // Unwrap data. Clean up resources held by pointer.
/// let userdata = unsafe {
///     Userdata::<u32>::consume(raw_data)
/// };
///
/// // Got user data. `raw_data` is no longer valid.
/// assert_eq!(userdata, 123);
/// ```
#[derive(Debug)]
pub struct Userdata<T>(PhantomData<T>);

impl<T> Userdata<T> {
    /// Wraps user data.
    ///
    /// This allocates memory. To prevent memory leaks, make sure to call [`consume()`] on the
    /// returned pointer exactly once.
    ///
    /// [`consume()`]: Self::consume
    pub fn prepare(userdata: T) -> *mut c_void {
        // Move `userdata` onto the heap and leak its memory into a raw pointer. This region will be
        // reclaimed later in `consume()`.
        let ptr: *mut T = Box::into_raw(Box::new(userdata));
        ptr.cast::<c_void>()
    }

    /// Wraps user data and returns sentinel.
    ///
    /// This uses [`prepare()`] but wraps the resulting pointer in [`UserdataSentinel`] to make sure
    /// that it is cleaned up when the sentinel is dropped. This is useful when the recipient of the
    /// raw pointer does not become the owner, i.e. does not clean up the user data by itself.
    ///
    /// Use [`UserdataSentinel::as_ptr()`] to get the pointer from the sentinel.
    ///
    /// [`prepare()`]: Self::prepare
    /// [`sentinel()`]: Self::sentinel
    pub fn prepare_sentinel(userdata: T) -> UserdataSentinel<T> {
        let data = Self::prepare(userdata);
        UserdataSentinel(data, PhantomData)
    }

    /// Unwraps [`c_void`] pointer to access data.
    ///
    /// # Safety
    ///
    /// The given pointer must have been returned from [`prepare()`], using the same value type `T`.
    /// It must not have been given to [`consume()`] yet.
    ///
    /// The lifetime of the returned reference is not allowed to extend past the next call to either
    /// [`peek_at()`] or [`consume()`] and must not outlive the lifetime of `T` itself. (In case the
    /// user data has been wrapped into a [`UserdataSentinel`], the sentinel must still be alive.)
    ///
    /// [`prepare()`]: Self::prepare
    /// [`peek_at()`]: Self::peek_at
    /// [`consume()`]: Self::consume
    pub unsafe fn peek_at<'a>(data: *mut c_void) -> &'a mut T {
        let ptr: *mut T = data.cast::<T>();
        // Reconstruct heap-allocated `userdata` back into its `Box`.
        let userdata = unsafe { Box::from_raw(ptr) };
        // Leak box as we do not want to destroy the data yet. This happens only when `consume()` is
        // called later.
        Box::leak(userdata)
    }

    /// Unwraps [`c_void`] pointer and returns owned data.
    ///
    /// # Safety
    ///
    /// The given pointer must have been returned from [`prepare()`], using the same value type `T`.
    /// It must not have been given to [`consume()`] yet, nor wrapped in a [`UserdataSentinel`] (the
    /// user data is consumed automatically when the sentinel is being dropped).
    ///
    /// [`prepare()`]: Self::prepare
    /// [`consume()`]: Self::consume
    /// [`sentinel()`]: Self::sentinel
    #[must_use]
    pub unsafe fn consume(data: *mut c_void) -> T {
        let ptr: *mut T = data.cast::<T>();
        // Reconstruct heap-allocated `userdata` back into its `Box`.
        let userdata = unsafe { Box::from_raw(ptr) };
        // TODO: Prefer `Box::into_inner()` when it becomes stable.
        // <https://github.com/rust-lang/rust/issues/80437>
        *userdata
    }
}

/// Sentinel for type-erased user data.
///
/// This consumes the user data when dropped.
#[derive(Debug)]
pub struct UserdataSentinel<T>(*mut c_void, PhantomData<T>);

impl<T> UserdataSentinel<T> {
    /// Gets underlying pointer from sentinel.
    ///
    /// This pointer can be passed to [`Userdata::peek_at()`] to get the user data. Usually, this is
    /// not done directly but through indirection (through the third-party library that required the
    /// use of pointers for raw context data in the first place).
    ///
    /// # Safety
    ///
    /// The sentinel remains the owner of the data. Care must be taken to not access the data in any
    /// way past the lifetime of the sentinel.
    #[must_use]
    pub const unsafe fn as_ptr(&self) -> *mut c_void {
        self.0
    }
}

// SAFETY: When `T` can be sent to another thread, the sentinel can be as well. (Despite the pointer
// inside, the sentinel is not a Rust reference but rather the owner of the data itself. To drop it,
// we move it out of its `Box` when consuming the `Userdata`.)
unsafe impl<T: Send> Send for UserdataSentinel<T> {}

impl<T> Drop for UserdataSentinel<T> {
    fn drop(&mut self) {
        let userdata = unsafe { Userdata::<T>::consume(self.0) };
        drop(userdata);
    }
}
