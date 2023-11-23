use std::{ptr, string::String as StdString};

use open62541_sys::{UA_String, UA_String_clear, UA_STRING_NULL};

pub struct String(UA_String);

impl String {
    #[must_use]
    pub fn new() -> Self {
        String(unsafe { UA_STRING_NULL })
    }

    #[must_use]
    pub fn to_string(&self) -> Option<StdString> {
        if self.0.length == 0 || self.0.data.is_null() {
            return Some(StdString::new());
        }

        let slice = unsafe { std::slice::from_raw_parts(self.0.data, self.0.length) };

        StdString::from_utf8(slice.into()).ok()
    }

    /// # Safety
    ///
    /// TODO
    #[must_use]
    pub unsafe fn as_mut(&mut self) -> *mut UA_String {
        ptr::addr_of_mut!(self.0)
    }
}

impl Drop for String {
    fn drop(&mut self) {
        unsafe { UA_String_clear(self.as_mut()) }
    }
}

impl Default for String {
    fn default() -> Self {
        String::new()
    }
}
