use std::{
    mem,
    ptr::{addr_of_mut, NonNull},
    slice,
};

use open62541_sys::{UA_Array_appendCopy, UA_Array_delete, UA_Array_new, UA_STATUSCODE_GOOD};

use crate::ua;

/// Wrapper for arrays from [`open62541_sys`].
///
/// This owns the wrapped data type. When the wrapper is dropped, its inner value is cleaned up with
/// [`UA_Array_delete()`].
#[allow(private_bounds)]
pub struct Array<T>(Option<(usize, NonNull<T::Inner>)>)
where
    T: ua::DataType;

#[allow(private_bounds)]
impl<T> Array<T>
where
    T: ua::DataType,
{
    #[must_use]
    pub fn new(size: usize) -> Option<Self> {
        debug_assert_eq!(
            unsafe { T::inner().as_ref() }.memSize() as usize,
            mem::size_of::<T>()
        );

        let array = NonNull::new(unsafe { UA_Array_new(size, T::inner().as_ptr()) })?;

        Some(Self(Some((size, array.cast()))))
    }

    pub fn from_slice(slice: &[T]) -> Option<Self> {
        let mut array = unsafe { UA_Array_new(0, T::inner().as_ptr()) };
        let mut size: usize = 0;

        for element in slice {
            let result = unsafe {
                UA_Array_appendCopy(
                    addr_of_mut!(array),
                    addr_of_mut!(size),
                    element.as_ptr().cast(),
                    T::inner().as_ptr(),
                )
            };

            if result != UA_STATUSCODE_GOOD {
                unsafe { UA_Array_delete(array, size, T::inner().as_ptr()) }

                return None;
            }
        }

        let Some(array) = NonNull::new(array) else {
            unsafe { UA_Array_delete(array, size, T::inner().as_ptr()) }

            return None;
        };

        Some(Self(Some((size, array.cast()))))
    }

    #[must_use]
    pub fn as_slice(&self) -> Option<&[T]> {
        let (size, ptr) = self.0?;

        Some(unsafe { slice::from_raw_parts(ptr.as_ptr().cast(), size) })
    }

    #[must_use]
    pub(crate) fn into_raw_parts(mut self) -> Option<(usize, *mut T::Inner)> {
        let (size, ptr) = self.0.take()?;

        Some((size, ptr.as_ptr()))
    }
}

impl<T> Drop for Array<T>
where
    T: ua::DataType,
{
    fn drop(&mut self) {
        if let Some((size, ptr)) = self.0 {
            unsafe { UA_Array_delete(ptr.as_ptr().cast(), size, T::inner().as_ptr()) }
        }
    }
}
