use std::{
    ffi::c_void,
    fmt, mem,
    num::NonZeroUsize,
    ops,
    ptr::{self, NonNull},
    slice,
};

use open62541_sys::{
    UA_Array_delete, UA_Array_new, UA_copy, UA_init, UA_EMPTY_ARRAY_SENTINEL, UA_STATUSCODE_GOOD,
};

use crate::DataType;

/// Wrapper for array from [`open62541_sys`].
///
/// This owns the wrapped data type. When the wrapper is dropped, its inner value is cleaned up with
/// [`UA_Array_delete()`] which also recursively cleans up all contained elements in the array.
///
/// Arrays in OPC UA can be in one of three states:
///
/// 1. Regular arrays with one or more elements
/// 2. Empty arrays of length zero
/// 3. Undefined arrays
///
/// This type tracks only the first two kinds of arrays. Thus when converting from raw parts, we may
/// return `None` to indicate that the given array is "undefined" (as specified by OPC UA).
pub struct Array<T: DataType>(State<T>);

/// Internal state of array.
///
/// This tracks whether the array contains any elements at all. Without elements, [`UA_Array_new()`]
/// would not return a pointer but the sentinel value [`UA_EMPTY_ARRAY_SENTINEL`]. We do not handle
/// these implications (mostly non-alignedness) but track these states explicitly instead.
enum State<T: DataType> {
    /// Array of length `0`.
    Empty,
    /// Array of the given length.
    NonEmpty {
        /// Pointer returned from [`UA_Array_new()`].
        ///
        /// This is always a valid pointer into one or more instances of `[T::Inner]`, i.e. non-null
        /// and not [`UA_EMPTY_ARRAY_SENTINEL`]. For empty arrays, we never call [`UA_Array_new()`]
        /// but use [`State::Empty`] instead.
        ptr: NonNull<T::Inner>,
        size: NonZeroUsize,
    },
}

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

        let array = NonNull::new(unsafe { UA_Array_new(size.get(), T::data_type()) })
            .expect("create new UA_Array")
            .cast::<T::Inner>();
        // We require a proper pointer for safe operation (even when we do not access the pointed-to
        // memory region at all, cf. documentation of `from_raw_parts_mut()`).
        debug_assert_ne!(array.as_ptr().cast::<c_void>().cast_const(), unsafe {
            UA_EMPTY_ARRAY_SENTINEL
        });

        // `UA_Array_new()` does not initialize any of its array elements. We do this manually here.
        //
        // SAFETY: `size` is non-zero, `array` is a valid pointer (not `UA_EMPTY_ARRAY_SENTINEL`).
        let slice: &mut [T::Inner] =
            unsafe { slice::from_raw_parts_mut(array.as_ptr(), size.get()) };
        for element in slice {
            unsafe { UA_init(ptr::from_mut(element).cast::<c_void>(), T::data_type()) }
        }

        Self(State::NonEmpty { ptr: array, size })
    }

    /// Creates new array from existing elements.
    ///
    /// This takes ownership of the elements from the given iterator.
    ///
    /// # Panics
    ///
    /// Enough memory must be available to allocate array.
    pub(crate) fn from_iter<I: Iterator<Item = T>>(iter: I) -> Self {
        // This creates a temporary copy by first collecting all elements into a `Vec` and then once
        // more copying elements from the `Vec` into the new array from `UA_Array_new()`.
        //
        // TODO: Avoid temporary copy. How to deal with unknown size of iterator when initializing a
        // new array to hold its elements?
        Self::from_slice(&iter.collect::<Vec<_>>())
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

        let array = NonNull::new(unsafe { UA_Array_new(slice.len(), T::data_type()) })
            .expect("create new UA_Array")
            .cast::<T::Inner>();
        // We require a proper pointer for safe operation (even when we do not access the pointed-to
        // memory region at all, cf. documentation of `from_raw_parts_mut()`).
        debug_assert_ne!(array.as_ptr().cast::<c_void>().cast_const(), unsafe {
            UA_EMPTY_ARRAY_SENTINEL
        });

        // Clone elements into the array. When this is done, all elements will be initialized. If we
        // need to stop because of an error, we may still call `UA_Array_delete()` because the array
        // elements have been zero-initialized by `UA_Array_new()` and `UA_Array_delete()` uses this
        // knowledge (under the hood, this is handled with a no-op of `UA_clear()` on each element).
        //
        // SAFETY: `size` is non-zero, `array` is a valid pointer (not `UA_EMPTY_ARRAY_SENTINEL`).
        let dst: &mut [T::Inner] = unsafe { slice::from_raw_parts_mut(array.as_ptr(), size.get()) };
        for (src, dst) in slice.iter().zip(dst) {
            let result = unsafe {
                UA_copy(
                    src.as_ptr().cast::<c_void>(),
                    ptr::from_mut(dst).cast::<c_void>(),
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
    #[must_use]
    pub(crate) fn from_raw_parts(ptr: *const T::Inner, size: usize) -> Option<Self> {
        if size == 0 {
            if ptr.is_null() {
                // This indicates an undefined array of unknown length. We do not handle this in the
                // type but return `None` instead.
                return None;
            }
            // Otherwise, we expect the sentinel value to indicate an empty array of length 0. This,
            // we do handle and may return `Some`.
            debug_assert_eq!(ptr.cast::<c_void>(), unsafe { UA_EMPTY_ARRAY_SENTINEL });
            return Some(Self(State::Empty));
        }

        // We require a proper pointer for safe operation (even when we do not access the pointed-to
        // memory region at all, cf. documentation of `from_raw_parts()`).
        debug_assert!(!ptr.is_null());
        debug_assert_ne!(ptr.cast::<c_void>(), unsafe { UA_EMPTY_ARRAY_SENTINEL });

        // Here we transmute the pointed-to elements from `T::Inner` to `T`. This is allowed because
        // `T` implements the trait `DataType`.
        //
        // SAFETY: `size` is non-zero, `array` is a valid pointer (not `UA_EMPTY_ARRAY_SENTINEL`).
        let slice = unsafe { slice::from_raw_parts(ptr.cast::<T>(), size) };
        Some(Self::from_slice(slice))
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        match self.0 {
            State::Empty => 0,
            State::NonEmpty { size, .. } => size.get(),
        }
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        match self.0 {
            State::Empty => true,
            State::NonEmpty { .. } => false,
        }
    }

    #[must_use]
    pub const fn as_slice(&self) -> &[T] {
        match self.0 {
            State::Empty => &[],

            State::NonEmpty { ptr, size } => {
                // We may return `&[T]` here instead of `&[T::Inner]` as `T: DataType` guarantees us
                // that we can transmute between the two types.
                unsafe { slice::from_raw_parts(ptr.as_ptr().cast::<T>(), size.get()) }
            }
        }
    }

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

    #[must_use]
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &T> {
        self.as_slice().iter()
    }

    #[must_use]
    pub fn iter_mut(&mut self) -> impl ExactSizeIterator<Item = &mut T> {
        self.as_slice_mut().iter_mut()
    }

    /// Consumes the array elements as an iterator.
    ///
    /// Replaces the elements of the array by default-initialized instances ([`DataType::init()`]).
    /// Ownership of the original elements is transferred to the resulting iterator items.
    ///
    /// Note: Other than [`Vec::drain()`], this method does _not_ shrink the array.
    // TODO: How to implement `IntoIterator` on `self` instead of `&mut self`?
    #[must_use]
    pub(crate) fn drain_all(&mut self) -> impl ExactSizeIterator<Item = T> + '_ {
        // This looks more expensive than it is: `DataType::init()` uses `UA_init()` which
        // zero-initializes the memory region left in place of the moved-out element. This
        // means that there are no dynamic memory allocations involved which would have to
        // be cleaned up when `self` is dropped. In fact, this is what `UA_Array_resize()`
        // does when making space for new elements, which in turn means that we can safely
        // rely on `UA_Array_delete()` to work correctly when it frees each dummy element.
        self.iter_mut()
            .map(|element| mem::replace(element, T::init()))
        // The resulting iterator contains all elements. The original elements in the array
        // have been replaced with zero-initialized memory. Dynamic memory allocations
        // held by the elements have not been touched, i.e. there is now (as before)
        // only a single owner.
    }

    /// Converts the array into a `Vec`.
    ///
    /// This avoids cloning the contained values and moves them into the `Vec` directly.
    #[must_use]
    pub fn into_vec(mut self) -> Vec<T> {
        self.drain_all().collect()
    }

    /// Converts the array into a native Rust array.
    ///
    /// This avoids cloning the contained values and moves them into the array directly. When the
    /// number of array elements does not match, this returns `None`.
    #[must_use]
    pub fn into_array<const N: usize>(self) -> Option<[T; N]> {
        <[T; N]>::try_from(self.into_vec()).ok()
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
                let ptr = unsafe { UA_EMPTY_ARRAY_SENTINEL };
                (ptr.cast::<T::Inner>().cast_mut(), 0)
            }
            State::NonEmpty { ptr, size } => (ptr.as_ptr(), size.get()),
        };

        // Make sure that `drop()` is not called anymore.
        mem::forget(self);
        (size, ptr)
    }

    /// Moves array into `dst`, giving up ownership.
    ///
    /// Existing data in `dst` is cleared with [`UA_Array_delete()`] before moving the value; it is
    /// safe to use this operation on already initialized target values.
    ///
    /// After this, it is the responsibility of `dst` to eventually clean up the data.
    pub(crate) fn move_into_raw(self, dst_size: &mut usize, dst: &mut *mut T::Inner) {
        // Make sure to clean up any previous value in target.
        let _unused = Self::from_raw_parts(*dst, *dst_size);

        let (size, ptr) = self.into_raw_parts();
        *dst_size = size;
        *dst = ptr;
    }
}

impl<T: DataType> Drop for Array<T> {
    fn drop(&mut self) {
        match self.0 {
            State::Empty => {
                // For empty arrays without allocation, we don't need to do anything here.
            }

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

// TODO
// impl<T: DataType> Clone for Array<T> {
//     fn clone(&self) -> Self {
//         todo!()
//     }
// }

impl<T: DataType> fmt::Debug for Array<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_slice().fmt(f)
    }
}

impl<T: DataType> ops::Index<usize> for Array<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl<T: DataType> ops::IndexMut<usize> for Array<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_slice_mut()[index]
    }
}

#[cfg(feature = "serde")]
impl<T: DataType + serde::Serialize> serde::Serialize for Array<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_seq(self.iter())
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    use open62541_sys::UA_NODEID_STRING_ALLOC;

    use crate::ua;

    use super::*;

    #[test]
    fn create_and_drop_array() {
        const STRING: &str = "LoremIpsum";
        const LEN: usize = 123;
        const POS: usize = 42;
        type T = ua::NodeId;

        // Create and drop array locally.
        //
        let mut array: Array<T> = Array::new(LEN);
        // Copy value with allocated data into array to catch double free.
        let target = array.as_slice_mut().get_mut(POS).unwrap();
        ua::NodeId::string(0, STRING).clone_into_raw(unsafe { target.as_mut() });

        drop(array);

        // Create array locally, delete in `open62541`.
        //
        let mut array: Array<T> = Array::new(LEN);
        // Copy value with allocated data into array to catch double free.
        let target = array.as_slice_mut().get_mut(POS).unwrap();
        ua::NodeId::string(0, STRING).clone_into_raw(unsafe { target.as_mut() });

        let (size, ptr) = array.into_raw_parts();
        assert_eq!(size, LEN);

        unsafe { UA_Array_delete(ptr.cast(), size, T::data_type()) }

        // Create array in `open62541`, delete locally.
        //
        let size = LEN;
        let ptr = unsafe { UA_Array_new(size, T::data_type()) }.cast::<<T as DataType>::Inner>();
        // Copy value with allocated data into array to catch double free.
        let string = CString::new(STRING).unwrap();
        *unsafe { ptr.add(POS).as_mut() }.unwrap() =
            unsafe { UA_NODEID_STRING_ALLOC(0, string.as_ptr()) };
        drop(string);

        let array: Array<T> = Array::from_raw_parts(ptr, size).unwrap();
        assert_eq!(array.len(), LEN);

        drop(array);
    }

    #[test]
    fn convert_array() {
        let array = ua::Array::from_slice(&[1, 2, 3].map(ua::Byte::new));
        let wrong: Option<[ua::Byte; 4]> = array.into_array();
        assert!(wrong.is_none());

        let array = ua::Array::from_slice(&[1, 2, 3].map(ua::Byte::new));
        let right: Option<[ua::Byte; 3]> = array.into_array();
        assert_eq!(
            Some([ua::Byte::new(1), ua::Byte::new(2), ua::Byte::new(3)]),
            right
        );
    }

    #[test]
    fn print_array() {
        let array = ua::Array::from_slice(&[1, 2, 3].map(ua::Byte::new));
        // Our implementation uses the default `Debug` representation of slices.
        assert_eq!("[1, 2, 3]", format!("{array:?}"));

        let array = ua::Array::from_slice(
            &["lorem", r#"ip"sum"#].map(|string| ua::String::new(string).unwrap()),
        );
        // String contents are automatically escaped, courtesy of `UA_print()`.
        assert_eq!(r#"["lorem", "ip\"sum"]"#, format!("{array:?}"));
    }
}
