use std::{ffi::CString, fmt, ptr, slice, str};

use open62541_sys::UA_String_fromChars;

use crate::{ArrayValue, Error};

crate::data_type!(String);

// In the implementation below, remember that `self.0.data` may be `UA_EMPTY_ARRAY_SENTINEL` for any
// strings of `length` 0. It may also be `ptr::null()` for "invalid" strings. This is similar to how
// OPC UA treats arrays (which also distinguishes between empty and invalid instances).
impl String {
    /// Creates string from string slice.
    ///
    /// # Errors
    ///
    /// The string must not contain any NUL bytes.
    pub fn new(s: &str) -> Result<Self, Error> {
        // We do not know for sure if `open62541` handles strings with contained NUL bytes correctly
        // in all situations. We avoid this entirely (at least for now). We may revisit this later.
        let src =
            CString::new(s).map_err(|_| Error::internal("string should not contain NUL bytes"))?;
        let str = unsafe { UA_String_fromChars(src.as_ptr()) };
        Ok(Self(str))
    }

    /// Creates invalid string (as defined by OPC UA).
    // TODO: The OPC UA specification calls invalid strings "null". Consider changing this to match.
    #[allow(dead_code)] // This is unused for now.
    pub(crate) fn invalid() -> Self {
        let str = unsafe { UA_String_fromChars(ptr::null()) };
        Self(str)
    }

    /// Creates empty string.
    #[allow(dead_code)] // This is unused for now.
    pub(crate) fn empty() -> Self {
        Self::new("").unwrap()
    }

    /// Checks if string is invalid.
    ///
    /// The invalid state is defined by OPC UA. It is a third state which is distinct from empty and
    /// regular (non-empty) strings.
    #[must_use]
    pub fn is_invalid(&self) -> bool {
        matches!(self.array_value(), ArrayValue::Invalid)
    }

    /// Checks if string is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        matches!(self.array_value(), ArrayValue::Empty)
    }

    #[deprecated(note = "use `Self::as_bytes()` instead")]
    #[must_use]
    pub fn as_slice(&self) -> Option<&[u8]> {
        self.as_bytes()
    }

    /// Returns string contents as byte slice.
    ///
    /// This may return [`None`] when the string itself is invalid (as defined by OPC UA).
    #[must_use]
    pub fn as_bytes(&self) -> Option<&[u8]> {
        // Internally, `open62541` represents strings as `Byte` array and has the same special cases
        // as regular arrays, i.e. empty and invalid states.
        match self.array_value() {
            ArrayValue::Invalid => None,
            ArrayValue::Empty => Some(&[]),
            ArrayValue::Valid(data) => {
                // `self.0.data` is valid, so we may use `self.0.length` now.
                Some(unsafe { slice::from_raw_parts(data.as_ptr(), self.0.length) })
            }
        }
    }

    /// Returns string contents as string slice.
    ///
    /// This may return [`None`] when the string itself is invalid (as defined by OPC UA) or when it
    /// is not valid Unicode (UTF-8).
    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        self.as_bytes().and_then(|slice| str::from_utf8(slice).ok())
    }

    fn array_value(&self) -> ArrayValue<u8> {
        // Internally, `open62541` represents strings as `Byte` array and has the same special cases
        // as regular arrays, i.e. empty and invalid states.
        ArrayValue::from_ptr(self.0.data)
    }
}

impl fmt::Display for String {
    /// Creates string from [`String`] value.
    ///
    /// # Examples
    ///
    /// ```
    /// use open62541::{ua, DataType as _};
    ///
    /// let node_id = ua::String::init();
    /// let str = node_id.to_string();
    ///
    /// assert_eq!(str, "");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display invalid strings as empty strings.
        f.write_str(self.as_str().unwrap_or(""))
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for String {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_str()
            .ok_or(serde::ser::Error::custom("String should be valid"))
            .and_then(|str| serializer.serialize_str(str))
    }
}

#[cfg(test)]
mod tests {
    use crate::ua;

    #[test]
    fn valid_string() {
        let str = ua::String::new("lorem ipsum").expect("should parse string");
        assert_eq!(str.as_str().expect("should display string"), "lorem ipsum");
        assert_eq!(str.to_string(), "lorem ipsum");
    }

    #[test]
    fn empty_string() {
        // Empty strings may have an internal representation in `UA_String` that contains invalid or
        // null pointers. This must not cause any problems.
        let str = ua::String::new("").expect("should parse empty string");
        assert_eq!(str.as_str().expect("should display empty string"), "");
        assert_eq!(str.to_string(), "");
    }
}
