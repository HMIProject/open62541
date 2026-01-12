use std::{fmt::Debug, mem, ptr};

use open62541_sys::{UA_DataType, UA_DataTypeArray};

/// Wrapper for data type array from [`open62541_sys`].
#[repr(transparent)]
pub struct DataTypeArray(UA_DataTypeArray);

impl DataTypeArray {
    /// Create new data type array.
    ///
    /// # Safety
    ///
    /// This aliases the given data types. The returned value _must_ be dropped before accessing the
    /// given data types again.
    pub(crate) unsafe fn new(data_types: &mut [UA_DataType]) -> Self {
        Self(UA_DataTypeArray {
            next: ptr::null_mut(),
            typesSize: data_types.len(),
            types: data_types.as_mut_ptr(),
            cleanup: false,
        })
    }

    /// Returns shared reference to value.
    ///
    /// # Safety
    ///
    /// The value is owned by `Self`. Ownership must not be given away, in whole or in parts. This
    /// may happen when `open62541` functions are called that take ownership of values by pointer.
    #[must_use]
    pub(crate) unsafe fn as_ref(&self) -> &UA_DataTypeArray {
        &self.0
    }

    #[must_use]
    pub(crate) fn as_ptr(&self) -> *const UA_DataTypeArray {
        &raw const self.0
    }

    #[must_use]
    pub(crate) fn into_raw(self) -> UA_DataTypeArray {
        // Use `ManuallyDrop` to avoid double-free even when added code might cause panic. See
        // documentation of `mem::forget()` for details.
        let this = mem::ManuallyDrop::new(self);
        // SAFETY: Aliasing memory temporarily is safe because destructor will not be called.
        unsafe { ptr::read(&raw const this.0) }
    }
}

impl Drop for DataTypeArray {
    fn drop(&mut self) {
        // For now, we only handle simple arrays that alias existing (owned) data types. So, we need
        // not run any cleanup here.
        assert!(self.0.next.is_null(), "unexpected next array");
        assert!(!self.0.cleanup, "unexpected array cleanup");
    }
}

impl Debug for DataTypeArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Add useful implementation.
        f.debug_tuple("DataTypeArray").finish_non_exhaustive()
    }
}
