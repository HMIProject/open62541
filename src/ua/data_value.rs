use std::{mem, ptr};

use open62541_sys::{
    UA_DataValue, UA_DataValue_clear, UA_DataValue_copy, UA_DataValue_init, UA_STATUSCODE_GOOD,
};

use crate::ua;

pub struct DataValue(UA_DataValue);

impl DataValue {
    #[allow(dead_code)]
    #[must_use]
    pub fn new() -> Self {
        let mut data_value = unsafe { mem::MaybeUninit::<UA_DataValue>::zeroed().assume_init() };
        unsafe { UA_DataValue_init(ptr::addr_of_mut!(data_value)) }
        Self(data_value)
    }

    /// Copies value from `src`.
    #[allow(dead_code)]
    pub(crate) fn from(src: &UA_DataValue) -> Self {
        let mut dst = Self::new();
        let result = unsafe { UA_DataValue_copy(src, dst.as_mut_ptr()) };
        assert_eq!(result, UA_STATUSCODE_GOOD);
        dst
    }

    /// Takes ownership of `src`.
    #[allow(dead_code)]
    pub(crate) fn from_inner(src: UA_DataValue) -> Self {
        DataValue(src)
    }

    #[allow(dead_code)]
    pub(crate) const fn as_ref(&self) -> &UA_DataValue {
        &self.0
    }

    #[allow(dead_code)]
    pub(crate) fn as_mut(&mut self) -> &mut UA_DataValue {
        &mut self.0
    }

    #[allow(dead_code)]
    pub(crate) const fn as_ptr(&self) -> *const UA_DataValue {
        ptr::addr_of!(self.0)
    }

    #[allow(dead_code)]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut UA_DataValue {
        ptr::addr_of_mut!(self.0)
    }

    #[allow(dead_code)]
    pub(crate) fn into_inner(self) -> UA_DataValue {
        let data_value = self.0;
        mem::forget(self);
        data_value
    }

    #[must_use]
    pub fn value(&self) -> ua::Variant {
        ua::Variant::from(&self.0.value)
    }
}

impl Drop for DataValue {
    fn drop(&mut self) {
        unsafe { UA_DataValue_clear(self.as_mut_ptr()) }
    }
}

impl Default for DataValue {
    fn default() -> Self {
        Self::new()
    }
}
