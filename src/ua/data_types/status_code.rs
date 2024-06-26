use std::{ffi::CStr, fmt};

use open62541_sys::{
    UA_StatusCode, UA_StatusCode_isBad, UA_StatusCode_isGood, UA_StatusCode_isUncertain,
    UA_StatusCode_name, UA_STATUSCODE_BADINTERNALERROR, UA_STATUSCODE_BADWRITENOTSUPPORTED,
    UA_STATUSCODE_GOOD,
};

crate::data_type!(StatusCode);

impl StatusCode {
    /// Enum variant [`UA_STATUSCODE_GOOD`] from [`open62541_sys`].
    pub const GOOD: Self = Self(UA_STATUSCODE_GOOD);

    /// Enum variant [`UA_STATUSCODE_BADINTERNALERROR`] from [`open62541_sys`].
    pub const BADINTERNALERROR: Self = Self(UA_STATUSCODE_BADINTERNALERROR);

    /// Enum variant [`UA_STATUSCODE_BADWRITENOTSUPPORTED`] from [`open62541_sys`].
    pub const BADWRITENOTSUPPORTED: Self = Self(UA_STATUSCODE_BADWRITENOTSUPPORTED);

    /// Creates wrapper by taking ownership of `src`.
    #[must_use]
    pub(crate) const fn new(src: UA_StatusCode) -> Self {
        Self(src)
    }

    /// Checks if status code is good.
    ///
    /// Good status codes indicate that the operation was successful and the associated results may
    /// be used.
    ///
    /// Note: This only checks the _severity_ of the status code. If you want to see if the code is
    /// exactly the single status code [`GOOD`](Self::GOOD), use comparison instead:
    ///
    /// ```
    /// use open62541::ua;
    ///
    /// # let status_code = ua::StatusCode::GOOD;
    /// if status_code == ua::StatusCode::GOOD {
    ///     //
    /// }
    /// ```
    #[must_use]
    pub fn is_good(&self) -> bool {
        unsafe { UA_StatusCode_isGood(self.0) }
    }

    /// Checks if status code is uncertain.
    ///
    /// Uncertain status codes indicate that the operation was partially successful and that
    /// associated results might not be suitable for some purposes.
    #[must_use]
    pub fn is_uncertain(&self) -> bool {
        unsafe { UA_StatusCode_isUncertain(self.0) }
    }

    /// Checks if status code is bad.
    ///
    /// Bad status codes indicate that the operation failed and any associated results cannot be
    /// used.
    #[must_use]
    pub fn is_bad(&self) -> bool {
        unsafe { UA_StatusCode_isBad(self.0) }
    }

    /// Gets name of status code.
    ///
    /// This returns the human-readable name of the status code, e.g. `BadNotWritable`.
    ///
    /// # Examples
    ///
    /// ```
    /// use open62541::ua;
    ///
    /// assert_eq!(ua::StatusCode::GOOD.name(), "Good");
    /// ```
    #[must_use]
    pub fn name(&self) -> &'static str {
        status_code_name(self.0)
    }
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// Gets statically allocated string with name of status code.
fn status_code_name(status_code: UA_StatusCode) -> &'static str {
    let name = unsafe { UA_StatusCode_name(status_code) };
    // SAFETY: `name` is a pointer to a valid NUL-terminated C string.
    // SAFETY: The string is statically allocated (for `'static`).
    // PANIC: The string contains only ASCII characters.
    unsafe { CStr::from_ptr(name) }.to_str().unwrap()
}
