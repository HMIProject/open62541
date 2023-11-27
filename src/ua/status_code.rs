use std::{ffi::CStr, fmt};

use open62541_sys::{UA_StatusCode, UA_StatusCode_name};

/// Wrapper for [`UA_StatusCode`] from [`open62541_sys`].
#[derive(Debug, Clone, Copy)]
pub struct StatusCode(UA_StatusCode);

impl StatusCode {
    /// Creates wrapper by taking ownership of `src`.
    #[must_use]
    pub fn new(src: UA_StatusCode) -> Self {
        Self(src)
    }
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = unsafe { CStr::from_ptr(UA_StatusCode_name(self.0)) };
        f.write_str(&String::from_utf8_lossy(name.to_bytes()))
    }
}
