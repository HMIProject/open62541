use std::{
    mem,
    ptr::{self, NonNull},
    slice,
};

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
            .expect("create new UA_Array");

        // `UA_Array_new` does not default-initialize the array elements.
        let slice: &mut [T::Inner] =
            unsafe { slice::from_raw_parts_mut(array.as_ptr().cast(), size) };
        for element in slice {
            unsafe { UA_init(ptr::addr_of_mut!(*element).cast(), T::data_type()) }
        }

        Self {
            ptr: array.cast(),
            size,
        }
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
            .expect("create new UA_Array");

        // Clone elements into the array. When this is done, all elements will be initialized. If we
        // need to stop because of an error, we may still call `UA_Array_delete()` because the array
        // elements have been zero-initialized by `UA_Array_new()` and `UA_Array_delete()` uses this
        // (under the hood, this is handled by `UA_clear()` on each element).
        let dst: &mut [T::Inner] =
            unsafe { slice::from_raw_parts_mut(array.as_ptr().cast(), slice.len()) };
        for (src, dst) in slice.iter().zip(dst) {
            let result = unsafe {
                UA_copy(
                    ptr::addr_of!(*src).cast(),
                    ptr::addr_of_mut!(*dst).cast(),
                    T::data_type(),
                )
            };

            if result != UA_STATUSCODE_GOOD {
                // When adding a single element fails, we clean up all elements that have been added
                // into the array up to this point. This is done by `UA_Array_delete()`. It can also
                // deal with elements that have not been initialized (cloned from `slice`) yet.
                unsafe { UA_Array_delete(array.as_ptr(), slice.len(), T::data_type()) }
                panic!("create new UA_Array")
            }
        }

        Self {
            ptr: array.cast(),
            size: slice.len(),
        }
    }

    #[allow(private_interfaces)]
    #[must_use]
    pub fn as_slice(&self) -> Option<&[T]> {
        // We may return `&[T]` here instead of `&[T::Inner]` because `T: DataType` guarantees us to
        // uphold the invariant that we can transmute between the two types.
        Some(unsafe { slice::from_raw_parts(self.ptr.as_ptr().cast(), self.size) })
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
        unsafe { UA_Array_delete(self.ptr.as_ptr().cast(), self.size, T::data_type()) }
    }
}
