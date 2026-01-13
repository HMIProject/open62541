use std::{fmt::Debug, iter, slice};

use open62541_sys::UA_DataTypeArray;

use crate::ua;

/// Wrapper for data type array from [`open62541_sys`].
#[repr(transparent)]
pub struct DataTypeArray(UA_DataTypeArray);

impl DataTypeArray {
    #[must_use]
    pub(crate) fn as_ptr(&self) -> *const UA_DataTypeArray {
        &raw const self.0
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &ua::DataType> {
        let mut array = &self.0;
        let mut index = 0;

        iter::from_fn(move || {
            loop {
                if index < array.typesSize {
                    let types = unsafe { slice::from_raw_parts(array.types, array.typesSize) };
                    let r#type = &types[index];
                    index += 1;
                    return Some(ua::DataType::raw_ref(r#type));
                }

                if array.next.is_null() {
                    return None;
                }

                // SAFETY: Points to valid instance of `UA_DataTypeArray`.
                array = unsafe { array.next.as_ref() }.unwrap();
                index = 0;
            }
        })
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
        f.debug_list().entries(self.iter()).finish()
    }
}
