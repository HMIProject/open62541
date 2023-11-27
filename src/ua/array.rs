use std::{ffi::c_void, mem, ptr::NonNull, slice};

use open62541_sys::{UA_Array_delete, UA_Array_new, UA_copy, UA_init, UA_STATUSCODE_GOOD};

use crate::DataType;

/// Wrapper for arrays from [`open62541_sys`].
///
/// This owns the wrapped data type. When the wrapper is dropped, its inner value is cleaned up with
/// [`UA_Array_delete()`] which also recursively cleans up all contained elements in the array.
#[allow(private_bounds)]
pub struct Array<T: DataType> {
    ptr: NonNull<T::Inner>,
    size: usize,
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
        debug_assert_eq!(T::data_type_ref().memSize() as usize, mem::size_of::<T>());
        let array = NonNull::new(unsafe { UA_Array_new(size, T::data_type()) })
            .expect("create new UA_Array")
            .cast::<T::Inner>();

        // `UA_Array_new` does not default-initialize the array elements. Do this manually here, one
        // element at a time.
        let slice: &mut [T::Inner] = unsafe { slice::from_raw_parts_mut(array.as_ptr(), size) };
        for element in slice {
            unsafe { UA_init((element as *mut T::Inner).cast::<c_void>(), T::data_type()) }
        }

        Self { ptr: array, size }
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
        debug_assert_eq!(T::data_type_ref().memSize() as usize, mem::size_of::<T>());
        let array = NonNull::new(unsafe { UA_Array_new(slice.len(), T::data_type()) })
            .expect("create new UA_Array")
            .cast::<T::Inner>();

        // Clone elements into the array. When this is done, all elements will be initialized. If we
        // need to stop because of an error, we may still call `UA_Array_delete()` because the array
        // elements have been zero-initialized by `UA_Array_new()` and `UA_Array_delete()` uses this
        // (under the hood, this is handled by `UA_clear()` on each element).
        let dst: &mut [T::Inner] =
            unsafe { slice::from_raw_parts_mut(array.as_ptr(), slice.len()) };
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
                    UA_Array_delete(array.as_ptr().cast::<c_void>(), slice.len(), T::data_type());
                }
                panic!("create new UA_Array")
            }
        }

        Self {
            ptr: array,
            size: slice.len(),
        }
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
        // Here we transmute the pointed-to elements from `T::Inner` to `T`. This is allowed because
        // `T` implements the trait `DataType`.
        let slice = unsafe { slice::from_raw_parts(ptr.cast::<T>(), size) };
        Self::from_slice(slice)
    }

    #[allow(private_interfaces)]
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        // We may return `&[T]` here instead of `&[T::Inner]` because `T: DataType` guarantees us to
        // uphold the invariant that we can transmute between the two types.
        unsafe { slice::from_raw_parts(self.ptr.as_ptr().cast::<T>(), self.size) }
    }

    #[allow(private_interfaces)]
    #[must_use]
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        // We may return `&[T]` here instead of `&[T::Inner]` because `T: DataType` guarantees us to
        // uphold the invariant that we can transmute between the two types.
        unsafe { slice::from_raw_parts_mut(self.ptr.as_ptr().cast::<T>(), self.size) }
    }

    #[must_use]
    pub(crate) fn into_raw_parts(self) -> (usize, *mut T::Inner) {
        let Self { ptr, size } = self;
        // Make sure that `drop()` is not called anymore.
        mem::forget(self);
        (size, ptr.as_ptr())
    }
}

impl<T: DataType> Drop for Array<T> {
    fn drop(&mut self) {
        // `UA_Array_delete` frees the heap-allocated array, along with any memory held by the array
        // elements.
        unsafe {
            UA_Array_delete(
                self.ptr.as_ptr().cast::<c_void>(),
                self.size,
                T::data_type(),
            );
        }
    }
}
