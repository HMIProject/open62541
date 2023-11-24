use std::{mem, ptr, slice};

use open62541_sys::{
    UA_ReadResponse, UA_ReadResponse_clear, UA_ReadResponse_copy, UA_ReadResponse_init,
    UA_STATUSCODE_GOOD,
};

use crate::ua;

pub struct ReadResponse(UA_ReadResponse);

impl ReadResponse {
    #[must_use]
    pub fn new() -> Self {
        let mut inner = unsafe { mem::MaybeUninit::<UA_ReadResponse>::zeroed().assume_init() };
        unsafe { UA_ReadResponse_init(ptr::addr_of_mut!(inner)) }
        Self(inner)
    }

    /// Copies value from `src`.
    #[allow(dead_code)]
    pub(crate) fn from(src: &UA_ReadResponse) -> Self {
        let mut dst = Self::new();

        let result = unsafe { UA_ReadResponse_copy(src, dst.as_mut_ptr()) };
        assert_eq!(result, UA_STATUSCODE_GOOD);

        dst
    }

    /// Takes ownership of `src`.
    #[allow(dead_code)]
    pub(crate) fn from_inner(src: UA_ReadResponse) -> Self {
        Self(src)
    }

    #[allow(dead_code)]
    pub(crate) const fn as_ref(&self) -> &UA_ReadResponse {
        &self.0
    }

    #[allow(dead_code)]
    pub(crate) fn as_mut(&mut self) -> &mut UA_ReadResponse {
        &mut self.0
    }

    #[allow(dead_code)]
    pub(crate) const fn as_ptr(&self) -> *const UA_ReadResponse {
        ptr::addr_of!(self.0)
    }

    #[allow(dead_code)]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut UA_ReadResponse {
        ptr::addr_of_mut!(self.0)
    }

    #[allow(dead_code)]
    pub(crate) fn into_inner(self) -> UA_ReadResponse {
        let inner = self.0;
        mem::forget(self);
        inner
    }
}

impl Drop for ReadResponse {
    fn drop(&mut self) {
        unsafe { UA_ReadResponse_clear(self.as_mut_ptr()) }
    }
}

impl Default for ReadResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl ReadResponse {
    #[must_use]
    pub fn results(&self) -> Vec<ua::DataValue> {
        let results = unsafe { slice::from_raw_parts(self.0.results, self.0.resultsSize) };
        results.iter().map(ua::DataValue::from).collect()
    }
}
