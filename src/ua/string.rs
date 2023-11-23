use std::{ptr, slice, str};

use open62541_sys::{UA_String, UA_String_clear, UA_STRING_NULL};

pub struct String(UA_String);

impl String {
    #[must_use]
    pub fn new() -> Self {
        Self(unsafe { UA_STRING_NULL })
    }

    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        let slice = unsafe { slice::from_raw_parts(self.0.data, self.0.length) };

        str::from_utf8(slice).ok()
    }

    #[must_use]
    pub const fn as_ptr(&self) -> *const UA_String {
        ptr::addr_of!(self.0)
    }

    #[must_use]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut UA_String {
        ptr::addr_of_mut!(self.0)
    }
}

impl Drop for String {
    fn drop(&mut self) {
        // `UA_String_clear` matches owned inner type.
        unsafe { UA_String_clear(self.as_mut_ptr()) }
    }
}

impl Default for String {
    fn default() -> Self {
        Self::new()
    }
}
