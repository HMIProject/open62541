use std::{
    mem,
    ptr::{addr_of, addr_of_mut},
};

use open62541_sys::{UA_ReadRequest, UA_ReadRequest_clear, UA_ReadRequest_init};

use crate::ua;

pub struct ReadRequest(UA_ReadRequest);

impl ReadRequest {
    #[must_use]
    pub fn new() -> Option<Self> {
        let mut request = unsafe { mem::MaybeUninit::<UA_ReadRequest>::zeroed().assume_init() };

        unsafe { UA_ReadRequest_init(addr_of_mut!(request)) }

        Some(Self(request))
    }

    #[must_use]
    pub fn nodes_to_read(mut self, nodes_to_read: &[ua::ReadValueId]) -> Option<Self> {
        let array = ua::Array::from_slice(nodes_to_read)?;

        let (size, ptr) = array.into_raw_parts()?;

        self.0.nodesToReadSize = size;
        self.0.nodesToRead = ptr;

        Some(self)
    }

    #[must_use]
    #[allow(dead_code)]
    pub(crate) const fn as_ptr(&self) -> *const UA_ReadRequest {
        addr_of!(self.0)
    }

    #[must_use]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut UA_ReadRequest {
        addr_of_mut!(self.0)
    }

    pub(crate) fn into_inner(self) -> UA_ReadRequest {
        let request = self.0;
        mem::forget(self);
        request
    }
}

impl Drop for ReadRequest {
    fn drop(&mut self) {
        // `UA_ReadRequest_clear` matches owned inner type.
        unsafe { UA_ReadRequest_clear(self.as_mut_ptr()) }
    }
}
