use std::{borrow::Cow, ffi::CString, fmt, slice, str, string};

use open62541_sys::UA_String_fromChars;

use crate::Error;

crate::data_type!(String);

// In the implementation below, remember that `self.0.data` may be `UA_EMPTY_ARRAY_SENTINEL` for any
// strings of `length` 0. It may also be `ptr::null()` for "invalid" strings. This is similar to how
// OPC UA treats arrays (which also distinguishes between empty and invalid instances).
impl String {
    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        // TODO: Handle `UA_EMPTY_ARRAY_SENTINEL` and `ptr::null()` correctly.
        let slice = unsafe { slice::from_raw_parts(self.0.data, self.0.length) };
        str::from_utf8(slice).ok()
    }

    #[must_use]
    pub fn to_string(&self) -> Cow<'_, str> {
        // TODO: Handle `UA_EMPTY_ARRAY_SENTINEL` and `ptr::null()` correctly.
        let slice = unsafe { slice::from_raw_parts(self.0.data, self.0.length) };
        string::String::from_utf8_lossy(slice)
    }
}

impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display invalid strings as empty strings.
        f.write_str(self.as_str().unwrap_or(""))
    }
}

impl str::FromStr for String {
    type Err = Error;

    /// Creates string from string slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use open62541::ua;
    ///
    /// let node_id: ua::String = "Lorem Ipsum".parse().unwrap();
    /// ```
    ///
    /// # Panics
    ///
    /// The string slice must not contain any NUL bytes.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // We do not know for sure if `open62541` handles strings with contained NUL bytes correctly
        // in all situations. We avoid this entirely (at least for now). We may revisit this later.
        let src =
            CString::new(s).map_err(|_| Error::internal("string should not contain NUL bytes"))?;
        let str = unsafe { UA_String_fromChars(src.as_ptr()) };
        Ok(Self(str))
    }
}

#[cfg(test)]
mod tests {
    use crate::ua;

    #[test]
    fn empty_string() {
        // Empty strings may have an internal representation in `UA_String` that contains invalid or
        // null pointers. This must not cause any problems.
        let str: ua::String = "".parse().expect("should parse empty string");
        assert_eq!(str.as_str().expect("should display empty string"), "");
        assert_eq!(str.to_string(), "");
    }

    #[test]
    fn valid_string() {
        let str: ua::String = "lorem ipsum".parse().expect("should parse string");
        assert_eq!(str.as_str().expect("should display string"), "lorem ipsum");
        assert_eq!(str.to_string(), "lorem ipsum");
    }
}
