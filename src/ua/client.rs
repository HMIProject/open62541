use std::ptr::NonNull;

use open62541_sys::{UA_Client, UA_Client_delete, UA_Client_new};

pub struct Client(NonNull<UA_Client>);

impl Client {
    #[must_use]
    pub fn new() -> Option<Self> {
        let client = NonNull::new(unsafe { UA_Client_new() })?;

        Some(Self(client))
    }

    #[must_use]
    pub fn as_ptr(&mut self) -> *mut UA_Client {
        self.0.as_ptr()
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        unsafe { UA_Client_delete(self.0.as_ptr()) }
    }
}
