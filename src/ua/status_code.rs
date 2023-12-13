use std::{ffi::CStr, fmt};

use open62541_sys::{UA_StatusCode, UA_StatusCode_name, UA_STATUSCODE_GOOD};

/// Wrapper for [`UA_StatusCode`] from [`open62541_sys`].
#[derive(Debug, Clone, Copy)]
pub struct StatusCode(UA_StatusCode);

impl StatusCode {
    /// Creates wrapper by taking ownership of `src`.
    #[must_use]
    pub const fn new(src: UA_StatusCode) -> Self {
        Self(src)
    }

    /// Checks if status code is good.
    #[must_use]
    pub(crate) const fn is_good(self) -> bool {
        // TODO: Check name. Consider potential clash with `UA_StatusCode_isGood()` which only makes
        // check for _severity_ of status code (i.e. may match an entire range of codes).
        self.0 == UA_STATUSCODE_GOOD
    }
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = unsafe { CStr::from_ptr(UA_StatusCode_name(self.0)) };
        f.write_str(&String::from_utf8_lossy(name.to_bytes()))
    }
}
