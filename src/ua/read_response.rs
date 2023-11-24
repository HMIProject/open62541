use std::{mem, ptr};

use open62541_sys::{UA_ReadResponse, UA_ReadResponse_clear};

pub struct ReadResponse(UA_ReadResponse);

impl ReadResponse {
    /// Takes ownership of `read_response`.
    pub(crate) fn new(read_response: UA_ReadResponse) -> Self {
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
}

impl Drop for ReadResponse {
    fn drop(&mut self) {
        unsafe { UA_ReadResponse_clear(self.as_mut_ptr()) }
    }
}
