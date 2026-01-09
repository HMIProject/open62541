use std::{fmt::Debug, ptr};

use open62541_sys::{UA_DataType, UA_DataTypeArray};

/// Wrapper for data type array from [`open62541_sys`].
#[repr(transparent)]
pub struct DataTypeArray(UA_DataTypeArray);

impl DataTypeArray {
    #[must_use]
    pub(crate) fn new(data_types: &mut [UA_DataType]) -> Self {
        Self(UA_DataTypeArray {
            next: ptr::null_mut(),
            typesSize: data_types.len(),
            types: data_types.as_mut_ptr(),
            cleanup: false,
        })
    }

    #[must_use]
    pub(crate) fn as_ptr(&self) -> *const UA_DataTypeArray {
        &raw const self.0
    }
}

impl Debug for DataTypeArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("DataTypeArray").finish_non_exhaustive()
    }
}
