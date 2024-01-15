use std::{ffi::CString, fmt, slice, str};

use open62541_sys::UA_String_fromChars;

use crate::{ua, Error};

crate::data_type!(String);

// In the implementation below, remember that `self.0.data` may be `UA_EMPTY_ARRAY_SENTINEL` for any
// strings of `length` 0. It may also be `ptr::null()` for "invalid" strings. This is similar to how
// OPC UA treats arrays (which also distinguishes between empty and invalid instances).
impl String {
    /// Returns string contents as slice.
    ///
    /// This may return [`None`] when the string itself is invalid (state as defined by OPC UA).
    #[must_use]
    pub fn as_slice(&self) -> Option<&[u8]> {
        match ua::PointerValue::from_raw(self.0.data) {
            ua::PointerValue::Invalid => None,
            ua::PointerValue::Empty => Some(&[]),
            ua::PointerValue::Valid(data) => {
                // `self.0.data` is valid, so we may use `self.0.length` now.
                Some(unsafe { slice::from_raw_parts(data.as_ptr(), self.0.length) })
            }
        }
    }

    /// Returns string as string slice.
    ///
    /// This may return [`None`] when the string itself is invalid or it is not valid UTF-8.
    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        self.as_slice().and_then(|slice| str::from_utf8(slice).ok())
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
