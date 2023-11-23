use std::ptr::NonNull;

use open62541_sys::{UA_ReadRequest, UA_ReadRequest_delete, UA_ReadRequest_new};

pub struct ReadRequest(NonNull<UA_ReadRequest>);

impl ReadRequest {
    #[must_use]
    pub fn new() -> Option<Self> {
        // `UA_ReadRequest_new` matches `UA_ReadRequest_delete`.
        let ua_read_value_id = NonNull::new(unsafe { UA_ReadRequest_new() })?;

        Some(Self(ua_read_value_id))
    }

    #[must_use]
    pub const fn as_ptr(&self) -> *const UA_ReadRequest {
        self.0.as_ptr()
    }

    #[must_use]
    pub fn as_mut_ptr(&mut self) -> *mut UA_ReadRequest {
        self.0.as_ptr()
    }
}

impl Drop for ReadRequest {
    fn drop(&mut self) {
        // `UA_ReadRequest_delete` matches `UA_ReadRequest_new`.
        unsafe { UA_ReadRequest_delete(self.as_mut_ptr()) }
    }
}
