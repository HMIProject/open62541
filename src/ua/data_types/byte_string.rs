use std::slice;

use open62541_sys::{UA_ByteString_copy, UA_String};

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
