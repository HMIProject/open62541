use std::{ffi::c_void, slice};

use open62541_sys::{UA_NumericRange, UA_EMPTY_ARRAY_SENTINEL};

use crate::ua;

/// Wrapper for [`UA_NumericRange`] from [`open62541_sys`].
#[repr(transparent)]
pub struct NumericRange(UA_NumericRange);

impl NumericRange {
    /// Creates wrapper reference from value.
    #[must_use]
    pub fn raw_ref(src: &UA_NumericRange) -> &Self {
        let src: *const UA_NumericRange = src;
        // This transmutes between the inner type and `Self` through `cast()`. `Self` guarantee that
        // we can transmute between it and its inner type, so this is okay.
        let ptr = src.cast::<Self>();
        // SAFETY: `Self` guarantees that we can transmute between `Self` and the inner type.
        let ptr = unsafe { ptr.as_ref() };
        // SAFETY: Pointer is valid (non-zero) because it comes from a reference.
        unsafe { ptr.unwrap_unchecked() }
    }

    #[must_use]
    pub fn dimensions(&self) -> Option<&[ua::NumericRangeDimension]> {
        let (ptr, size) = (self.0.dimensions, self.0.dimensionsSize);
        if size == 0 {
            if ptr.is_null() {
                // This indicates an undefined array of unknown length. We do not handle this in the
                // type but return `None` instead.
                return None;
            }
            // Otherwise, we expect the sentinel value to indicate an empty array of length 0. This,
            // we do handle and may return `Some`.
            debug_assert_eq!(ptr.cast::<c_void>().cast_const(), unsafe {
                UA_EMPTY_ARRAY_SENTINEL
            });
            return Some(&[]);
        }

        // We require a proper pointer for safe operation (even when we do not access the pointed-to
        // memory region at all, cf. documentation of `from_raw_parts()`).
        debug_assert!(!ptr.is_null());
        debug_assert_ne!(ptr.cast::<c_void>().cast_const(), unsafe {
            UA_EMPTY_ARRAY_SENTINEL
        });

        // Here we transmute the pointed-to elements from `UA_NumericRangeDimension` to
        // `ua::NumericRangeDimension`. This is allowed because `ua::NumericRangeDimension` uses
        // `#[repr(transparent)]`.
        //
        // SAFETY: `size` is non-zero, `array` is a valid pointer (not `UA_EMPTY_ARRAY_SENTINEL`).
        let slice = unsafe { slice::from_raw_parts(ptr.cast::<ua::NumericRangeDimension>(), size) };
        Some(slice)
    }
}
