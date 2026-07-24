use std::fmt;
use std::ptr;
use std::str::FromStr;

use open62541_sys::UA_RelativePath_parse;
use open62541_sys::UA_RelativePath_print;

use crate::{DataType as _, Error, ua};

crate::data_type!(RelativePath);

impl RelativePath {
    #[must_use]
    pub fn with_elements(mut self, elements: &[ua::RelativePathElement]) -> Self {
        let array = ua::Array::from_slice(elements);
        array.move_into_raw(&mut self.0.elementsSize, &mut self.0.elements);
        self
    }

    #[must_use]
    pub fn elements(&self) -> Option<&[ua::RelativePathElement]> {
        // SAFETY: Lifetime of the slice is implicitly bound to the lifetime of the reference to self.
        // Pointer validity is checked in `Array::slice_from_raw_parts()`.
        unsafe { ua::Array::slice_from_raw_parts(self.0.elementsSize, self.0.elements) }
    }

    #[must_use]
    pub fn elements_mut(&mut self) -> Option<&mut [ua::RelativePathElement]> {
        // SAFETY: Lifetime of the slice is implicitly bound to the lifetime of the reference to self.
        // Pointer validity is checked in `Array::slice_from_raw_parts_mut()`.
        unsafe { ua::Array::slice_from_raw_parts_mut(self.0.elementsSize, self.0.elements) }
    }

    /// Returns the number of elements in the relative path.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.0.elementsSize
    }

    /// Returns true if the relative path has no elements.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0.elementsSize == 0
    }

    /// Parses a relative path from a string, using the limited grammar implemented by open62541.
    ///
    /// # Errors
    /// Will return an error if the string is not parseable as a relative path.
    ///
    /// See [UA_RelativePath_parse](https://open62541.org/doc/master/util.html#example-relativepaths) docs.
    pub fn parse(path: &str) -> Result<Self, Error> {
        let path = ua::String::new(path)?;
        let mut slf = Self::init();

        let result = unsafe { UA_RelativePath_parse(slf.as_mut_ptr(), ptr::read(path.as_ptr())) };

        let status_code = ua::StatusCode::new(result);

        if status_code.is_good() {
            Ok(slf)
        } else {
            Err(Error::new(status_code))
        }
    }
}

impl FromStr for RelativePath {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl fmt::Display for RelativePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = ua::String::null();

        let status_code = ua::StatusCode::new(unsafe {
            // SAFETY: `UA_RelativePath_print` will initialize the string if null.
            // See UA_RelativePath_print docs (https://open62541.org/doc/master/util.html#example-relativepaths)
            UA_RelativePath_print(&raw const self.0, str.as_mut_ptr())
        });

        // A lot of chained `UA_String_append` & `UA_String_escapeAppend`
        // calls + an optimistic stack allocation w/ retry if the buffer is too small.
        // SAFETY: `UA_RelativePath_print` will only return an error on a failed allocation.
        debug_assert!(
            status_code.is_good(),
            "failed to print relative path: {status_code}"
        );

        write!(f, "{str}")
    }
}
