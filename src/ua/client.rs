use std::ptr::NonNull;

use open62541_sys::{UA_Client, UA_Client_delete, UA_Client_new};

pub struct Client(NonNull<UA_Client>);

impl Client {
    #[must_use]
    pub fn new() -> Option<Self> {
        // `UA_Client_new` matches `UA_Client_delete`.
        let ua_client = NonNull::new(unsafe { UA_Client_new() })?;

        Some(Self(ua_client))
    }

    #[must_use]
    #[allow(dead_code)]
    pub(crate) const fn as_ptr(&self) -> *const UA_Client {
        self.0.as_ptr()
    }

    #[must_use]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut UA_Client {
        self.0.as_ptr()
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        // `UA_Client_delete` matches `UA_Client_new`.
        unsafe { UA_Client_delete(self.as_mut_ptr()) }
    }
}
