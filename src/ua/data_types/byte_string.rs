use std::slice;

use open62541_sys::{UA_ByteString_clear, UA_ByteString_copy, UA_ByteString_memZero, UA_String};

use crate::{ua, ArrayValue, DataType};

// Technically, `open62541_sys::ByteString` is an alias for `open62541_sys::String`. But we treat it
// as a distinct type to improve type safety. The difference is that `String` contains valid Unicode
// whereas `ByteString` may contain arbitrary byte sequences.
crate::data_type!(ByteString);

// In the implementation below, remember that `self.0.data` may be `UA_EMPTY_ARRAY_SENTINEL` for any
// strings of `length` 0. It may also be `ptr::null()` for "invalid" strings. This is similar to how
// OPC UA treats arrays (which also distinguishes between empty and invalid instances).
impl ByteString {
    /// Creates byte string from data.
    ///
    /// # Panics
    ///
    /// Enough memory must be available to copy data to the heap.
    #[must_use]
    pub fn new(s: &[u8]) -> Self {
        let mut dst = Self::init();
        let src = UA_String {
            length: s.len(),
            // SAFETY: `UA_String` needs `*mut UA_Byte`. But the call to `UA_ByteString_copy()` does
            // not actually mutate the value, it only reads from it.
            data: s.as_ptr().cast_mut(),
        };
        // We let `UA_ByteString_copy()` do the heavy lifting of allocating memory and copying data.
        let status_code =
            ua::StatusCode::new(unsafe { UA_ByteString_copy(&src, dst.as_mut_ptr()) });
        // PANIC: The only possible errors here are out-of-memory.
        assert!(
            status_code.is_good(),
            "byte string should have been created"
        );
        dst
    }

    #[allow(dead_code)] // --no-default-features
    fn clear(&mut self) {
        unsafe {
            // Clearing frees the referenced heap memory and resets length and data pointer to all
            // zeroes, i.e. the string becomes an "invalid" string (as defined by OPC UA).
            UA_ByteString_clear(self.as_mut_ptr());
        }
    }

    #[allow(dead_code)] // --no-default-features
    fn mem_zero(&mut self) {
        unsafe {
            // This zeroizes the string contents, i.e. characters, leaving the string object itself
            // intact. The string has the same length as before but is all `\0`.
            UA_ByteString_memZero(self.as_mut_ptr());
        }
    }

    /// Checks if byte string is invalid.
    ///
    /// The invalid state is defined by OPC UA. It is a third state which is distinct from empty and
    /// regular (non-empty) byte strings.
    #[must_use]
    pub fn is_invalid(&self) -> bool {
        matches!(self.array_value(), ArrayValue::Invalid)
    }

    /// Checks if byte string is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        matches!(self.array_value(), ArrayValue::Empty)
    }

    /// Returns byte string contents as slice.
    ///
    /// This may return [`None`] when the byte string itself is invalid (as defined by OPC UA).
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

    /// Returns byte string contents as slice, without checking for validity.
    ///
    /// # Panic
    ///
    /// The byte string itself must not be invalid (as defined by OPC UA).
    #[allow(dead_code)] // --no-default-features
    pub(crate) unsafe fn as_bytes_unchecked(&self) -> &[u8] {
        unsafe { self.as_bytes().unwrap_unchecked() }
    }

    fn array_value(&self) -> ArrayValue<u8> {
        // Internally, `open62541` represents strings as `Byte` array and has the same special cases
        // as regular arrays, i.e. empty and invalid states.
        ArrayValue::from_ptr(self.0.data)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for ByteString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_bytes()
            .ok_or(serde::ser::Error::custom("String should be valid"))
            .and_then(|bytes| serializer.serialize_bytes(bytes))
    }
}

#[cfg(feature = "mbedtls")]
impl zeroize::Zeroize for ua::ByteString {
    fn zeroize(&mut self) {
        // Clear the heap memory of the string characters.
        self.mem_zero();
        // Clear the string data structure, i.e. length.
        self.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zeroizing() {
        let mut string = ua::ByteString::new("SECRET".as_bytes());
        let data_ptr = string.as_bytes().unwrap().as_ptr();
        let object_ptr = unsafe { string.as_ptr() };

        let data = unsafe { slice::from_raw_parts(data_ptr, 6) };
        assert_eq!(data, "SECRET".as_bytes());
        let object = unsafe { std::ptr::read(object_ptr) };
        assert!(!object.data.is_null());
        assert_eq!(object.length, 6);

        string.mem_zero();

        // SAFETY: Memory has been zeroized but is still allocated.
        let data = unsafe { slice::from_raw_parts(data_ptr, 6) };
        assert_eq!(data, "\0\0\0\0\0\0".as_bytes());

        string.clear();

        // SAFETY: Object has been reset but is still allocated.
        let object = unsafe { std::ptr::read(object_ptr) };
        assert!(object.data.is_null());
        assert_eq!(object.length, 0);
    }
}
