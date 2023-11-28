use std::{ffi::c_void, mem, num::NonZeroUsize, ptr::NonNull, slice};

use open62541_sys::{
    UA_Array_delete, UA_Array_new, UA_copy, UA_init, UA_EMPTY_ARRAY_SENTINEL_, UA_STATUSCODE_GOOD,
};

use crate::DataType;

/// Wrapper for arrays from [`open62541_sys`].
///
/// This owns the wrapped data type. When the wrapper is dropped, its inner value is cleaned up with
/// [`UA_Array_delete()`] which also recursively cleans up all contained elements in the array.
#[allow(private_bounds)]
pub struct Array<T: DataType>(State<T>);

/// Internal state of array.
///
/// This tracks whether the array contains any elements at all. Without elements, [`UA_Array_new()`]
/// would not return a pointer but the sentinel value [`UA_EMPTY_ARRAY_SENTINEL_`]. We do not handle
/// the implications of this (mostly non-alignedness) and use explicit separate states instead.
enum State<T: DataType> {
    /// Array of length `0`.
    Empty,
    /// Array of the given length.
    NonEmpty {
        /// Pointer returned from [`UA_Array_new()`].
        ///
        /// This is always a valid pointer into one or more instances of `[T::Inner]`, i.e. non-null
        /// and not [`UA_EMPTY_ARRAY_SENTINEL_`]. For empty arrays, we never call [`UA_Array_new()`]
        /// but use [`State::Empty`] instead.
        ptr: NonNull<T::Inner>,
        size: NonZeroUsize,
    },
}

#[allow(private_bounds)]
impl<T: DataType> Array<T> {
    /// Creates new array with default-initialized elements.
    ///
    /// # Panics
    ///
    /// Enough memory must be available to allocate array.
    #[must_use]
    pub fn new(size: usize) -> Self {
        let Some(size) = NonZeroUsize::new(size) else {
            return Self(State::Empty);
        };

        debug_assert_eq!(T::data_type_ref().memSize() as usize, mem::size_of::<T>());
        let array = NonNull::new(unsafe { UA_Array_new(size.get(), T::data_type()) })
            .expect("create new UA_Array")
            .cast::<T::Inner>();
        // We require a proper pointer for safe operation (even when we do not access the pointed-to
        // memory region at all, cf. documentation of `from_raw_parts_mut()`).
        debug_assert_ne!(array.as_ptr().cast::<c_void>().cast_const(), unsafe {
            UA_EMPTY_ARRAY_SENTINEL_
        });

        // `UA_Array_new()` does not initialize any of its array elements. We do this manually here.
        //
        // SAFETY: `size` is non-zero, `array` is a valid pointer (not `UA_EMPTY_ARRAY_SENTINEL_`).
        let slice: &mut [T::Inner] =
            unsafe { slice::from_raw_parts_mut(array.as_ptr(), size.get()) };
        for element in slice {
            unsafe { UA_init((element as *mut T::Inner).cast::<c_void>(), T::data_type()) }
        }

        Self(State::NonEmpty { ptr: array, size })
    }

    /// Creates new array from existing elements.
    ///
    /// This copies over the elements from the given slice. The array will own the copies, and clean
    /// up when it is dropped. The original elements in the slice are left untouched.
    ///
    /// # Panics
    ///
    /// Enough memory must be available to allocate array.
    pub fn from_slice(slice: &[T]) -> Self {
        let Some(size) = NonZeroUsize::new(slice.len()) else {
            return Self(State::Empty);
        };

        debug_assert_eq!(T::data_type_ref().memSize() as usize, mem::size_of::<T>());
        let array = NonNull::new(unsafe { UA_Array_new(slice.len(), T::data_type()) })
            .expect("create new UA_Array")
            .cast::<T::Inner>();
        // We require a proper pointer for safe operation (even when we do not access the pointed-to
        // memory region at all, cf. documentation of `from_raw_parts_mut()`).
        debug_assert_ne!(array.as_ptr().cast::<c_void>().cast_const(), unsafe {
            UA_EMPTY_ARRAY_SENTINEL_
        });

        // Clone elements into the array. When this is done, all elements will be initialized. If we
        // need to stop because of an error, we may still call `UA_Array_delete()` because the array
        // elements have been zero-initialized by `UA_Array_new()` and `UA_Array_delete()` uses this
        // knowledge (under the hood, this is handled with a no-op of `UA_clear()` on each element).
        //
        // SAFETY: `size` is non-zero, `array` is a valid pointer (not `UA_EMPTY_ARRAY_SENTINEL_`).
        let dst: &mut [T::Inner] = unsafe { slice::from_raw_parts_mut(array.as_ptr(), size.get()) };
        for (src, dst) in slice.iter().zip(dst) {
            let result = unsafe {
                UA_copy(
                    src.as_ptr().cast::<c_void>(),
                    (dst as *mut T::Inner).cast::<c_void>(),
                    T::data_type(),
                )
            };

            if result != UA_STATUSCODE_GOOD {
                // When adding a single element fails, we clean up all elements that have been added
                // into the array up to this point. This is done by `UA_Array_delete()`, which knows
                // how to deal with elements that have not been initialized yet.
                unsafe {
                    UA_Array_delete(array.as_ptr().cast::<c_void>(), size.get(), T::data_type());
                }
                panic!("create new UA_Array")
            }
        }

        Self(State::NonEmpty { ptr: array, size })
    }

    /// Crates new array by copying existing raw parts.
    ///
    /// This may be used when items need to be copied out of a structure with attributes for pointer
    /// and size of the included array.
    ///
    /// # Panics
    ///
    /// Enough memory must be available to allocate array.
    #[allow(private_interfaces)]
    #[must_use]
    pub fn from_raw_parts(ptr: *const T::Inner, size: usize) -> Self {
        if size == 0 {
            debug_assert_eq!(ptr.cast::<c_void>(), unsafe { UA_EMPTY_ARRAY_SENTINEL_ });
            return Self(State::Empty);
        }

        // We require a proper pointer for safe operation (even when we do not access the pointed-to
        // memory region at all, cf. documentation of `from_raw_parts()`).
        debug_assert_ne!(ptr.cast::<c_void>(), unsafe { UA_EMPTY_ARRAY_SENTINEL_ });
        // Here we transmute the pointed-to elements from `T::Inner` to `T`. This is allowed because
        // `T` implements the trait `DataType`.
        //
        // SAFETY: `size` is non-zero, `array` is a valid pointer (not `UA_EMPTY_ARRAY_SENTINEL_`).
        let slice = unsafe { slice::from_raw_parts(ptr.cast::<T>(), size) };
        Self::from_slice(slice)
    }

    #[allow(private_interfaces)]
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        match self.0 {
            State::Empty => &[],

            State::NonEmpty { ptr, size } => {
                // We may return `&[T]` here instead of `&[T::Inner]` as `T: DataType` guarantees us
                // that we can transmute between the two types.
                unsafe { slice::from_raw_parts(ptr.as_ptr().cast::<T>(), size.get()) }
            }
        }
    }

    #[allow(private_interfaces)]
    #[must_use]
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        match self.0 {
            State::Empty => &mut [],

            State::NonEmpty { ptr, size } => {
                // We may return `&[T]` here instead of `&[T::Inner]` as `T: DataType` guarantees us
                // that we can transmute between the two types.
                unsafe { slice::from_raw_parts_mut(ptr.as_ptr().cast::<T>(), size.get()) }
            }
        }
    }

    /// Gives up ownership and returns raw parts.
    ///
    /// The returned raw parts must be deallocated with [`UA_Array_delete()`] to prevent leaking any
    /// memory. Alternatively, they may be re-wrapped by [`from_raw_parts()`](Self::from_raw_parts).
    #[must_use]
    pub(crate) fn into_raw_parts(self) -> (usize, *mut T::Inner) {
        let (ptr, size) = match self.0 {
            State::Empty => {
                // `UA_Array_new()` would return the sentinel value when "allocating" a size of `0`.
                // We emulate this here to allow `UA_Array_delete()` and other functions to use that
                // and handle this case appropriately (essentially making deallocation a no-op).
                let ptr = unsafe { UA_EMPTY_ARRAY_SENTINEL_ };
                (ptr.cast::<T::Inner>().cast_mut(), 0)
            }
            State::NonEmpty { ptr, size } => (ptr.as_ptr(), size.get()),
        };

        // Make sure that `drop()` is not called anymore.
        mem::forget(self);
        (size, ptr)
    }
}

impl<T: DataType> Drop for Array<T> {
    fn drop(&mut self) {
        match self.0 {
            State::Empty => {}
            State::NonEmpty { ptr, size } => {
                // `UA_Array_delete()` frees the heap-allocated array, along with any memory held by
                // the array elements.
                unsafe {
                    UA_Array_delete(ptr.as_ptr().cast::<c_void>(), size.get(), T::data_type());
                }
            }
        }
    }
}
