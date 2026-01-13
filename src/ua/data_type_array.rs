use std::fmt::Debug;

use open62541_sys::UA_DataTypeArray;

/// Wrapper for data type array from [`open62541_sys`].
#[repr(transparent)]
pub struct DataTypeArray(UA_DataTypeArray);

impl DataTypeArray {
    #[must_use]
    pub(crate) fn as_ptr(&self) -> *const UA_DataTypeArray {
        &raw const self.0
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
