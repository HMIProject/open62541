use std::{mem, ptr, slice};

use open62541_sys::{UA_ReadResponse, UA_ReadResponse_clear, UA_ReadResponse_init};

use crate::ua;

pub struct ReadResponse(UA_ReadResponse);

impl ReadResponse {
    #[must_use]
    pub fn new() -> Self {
        let mut read_response =
            unsafe { mem::MaybeUninit::<UA_ReadResponse>::zeroed().assume_init() };
        unsafe { UA_ReadResponse_init(ptr::addr_of_mut!(read_response)) }
        Self(read_response)
    }

    /// Takes ownership of `read_response`.
    pub(crate) fn from(read_response: UA_ReadResponse) -> Self {
        ReadResponse(read_response)
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
        let read_response = self.0;
        mem::forget(self);
        read_response
    }

    #[must_use]
    pub fn results(&self) -> Vec<ua::DataValue> {
        let results = unsafe { slice::from_raw_parts(self.0.results, self.0.resultsSize) };

        results.iter().map(ua::DataValue::from).collect()
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
