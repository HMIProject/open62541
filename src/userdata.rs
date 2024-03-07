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
pub struct Userdata<T>(PhantomData<T>);

impl<T> Userdata<T> {
    /// Wraps user data.
    ///
    /// This allocates memory. To prevent memory leaks, ensure to call [`consume()`] on the returned
    /// pointer exactly once.
    ///
    /// [`consume()`]: Self::consume
    pub fn prepare(userdata: T) -> *mut c_void {
        // Move `userdata` onto the heap and leak its memory into a raw pointer. This region will be
        // reclaimed later in `consume()`.
        let ptr: *mut T = Box::into_raw(Box::new(userdata));
        ptr.cast::<c_void>()
    }

    /// Unwraps [`c_void`] pointer to access data.
    ///
    /// # Safety
    ///
    /// The given pointer must have been returned from [`prepare()`], using the same value type `T`.
    /// It must not have been given to [`consume()`] yet.
    ///
    /// The lifetime of the returned reference is not allowed to extend past the next call to either
    /// [`peek_at()`] or [`consume()`] and must not outlive the lifetime of `T` itself.
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

    /// Unwraps [`c_void`] pointer and release memory.
    ///
    /// # Safety
    ///
    /// The given pointer must have been returned from [`prepare()`], using the same value type `T`.
    /// It must not have been given to [`consume()`] yet.
    ///
    /// [`prepare()`]: Self::prepare
    /// [`peek_at()`]: Self::peek_at
    /// [`consume()`]: Self::consume
    pub unsafe fn consume(data: *mut c_void) -> T {
        let ptr: *mut T = data.cast::<T>();
        // Reconstruct heap-allocated `userdata` back into its `Box`.
        let userdata = unsafe { Box::from_raw(ptr) };
        // TODO: Prefer `Box::into_inner()` when it becomes stable.
        // https://github.com/rust-lang/rust/issues/80437
        *userdata
    }
}
