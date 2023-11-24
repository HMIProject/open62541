use std::{mem, ptr};

use open62541_sys::{
    UA_ReadRequest, UA_ReadRequest_clear, UA_ReadRequest_copy, UA_ReadRequest_init,
    UA_STATUSCODE_GOOD,
};

use crate::ua;

pub struct ReadRequest(UA_ReadRequest);

impl ReadRequest {
    #[must_use]
    pub fn new() -> Self {
        let mut inner = unsafe { mem::MaybeUninit::<UA_ReadRequest>::zeroed().assume_init() };
        unsafe { UA_ReadRequest_init(ptr::addr_of_mut!(inner)) }
        Self(inner)
    }

    /// Copies value from `src`.
    #[allow(dead_code)]
    pub(crate) fn from(src: &UA_ReadRequest) -> Self {
        let mut dst = Self::new();

        let result = unsafe { UA_ReadRequest_copy(src, dst.as_mut_ptr()) };
        assert_eq!(result, UA_STATUSCODE_GOOD);

        dst
    }

    /// Takes ownership of `src`.
    #[allow(dead_code)]
    pub(crate) fn from_inner(src: UA_ReadRequest) -> Self {
        Self(src)
    }

    #[allow(dead_code)]
    pub(crate) const fn as_ref(&self) -> &UA_ReadRequest {
        &self.0
    }

    #[allow(dead_code)]
    pub(crate) fn as_mut(&mut self) -> &mut UA_ReadRequest {
        &mut self.0
    }

    #[allow(dead_code)]
    pub(crate) const fn as_ptr(&self) -> *const UA_ReadRequest {
        ptr::addr_of!(self.0)
    }

    #[allow(dead_code)]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut UA_ReadRequest {
        ptr::addr_of_mut!(self.0)
    }

    #[allow(dead_code)]
    pub(crate) fn into_inner(self) -> UA_ReadRequest {
        let inner = self.0;
        mem::forget(self);
        inner
    }
}

impl Drop for ReadRequest {
    fn drop(&mut self) {
        unsafe { UA_ReadRequest_clear(self.as_mut_ptr()) }
    }
}

impl Default for ReadRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl ReadRequest {
    #[must_use]
    pub fn nodes_to_read(mut self, nodes_to_read: &[ua::ReadValueId]) -> Option<Self> {
        let array = ua::Array::from_slice(nodes_to_read)?;

        let (size, ptr) = array.into_raw_parts()?;
        self.0.nodesToReadSize = size;
        self.0.nodesToRead = ptr;

        Some(self)
    }
}
