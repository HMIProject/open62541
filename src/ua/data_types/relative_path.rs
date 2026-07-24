use std::ptr;

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
        unsafe {
            ua::Array::slice_from_raw_parts(self.0.elementsSize, self.0.elements)
        }
    }

    #[must_use]
    pub fn elements_mut(&mut self) -> Option<&mut [ua::RelativePathElement]> {
        // SAFETY: Lifetime of the slice is implicitly bound to the lifetime of the reference to self.
        // Pointer validity is checked in `Array::slice_from_raw_parts_mut()`.
        unsafe {
            ua::Array::slice_from_raw_parts_mut(self.0.elementsSize, self.0.elements)
        }
    }

    pub fn len(&self) -> usize {
        self.0.elementsSize
    }

    pub fn is_empty(&self) -> bool {
        self.0.elementsSize == 0
    }

    pub fn parse(path: &str) -> Result<Self, Error> {
        let path = ua::String::new(path)?;
        let mut slf = Self::init();

        let result = unsafe {
            UA_RelativePath_parse(slf.as_mut_ptr(), ptr::read(path.as_ptr()))
        };

        let status_code = ua::StatusCode::new(result);

        if status_code.is_good() {
            Ok(slf)
        } else {
            Err(Error::new(status_code))
        }
    }

    pub fn to_string(&self) -> Result<String, Error> {
        let mut str = ua::String::null();

        let status_code = ua::StatusCode::new(unsafe {
            // SAFETY: `UA_RelativePath_print` will initialize the string if null.
            // See UA_RelativePath_print docs (https://open62541.org/doc/master/util.html#example-relativepaths)
            UA_RelativePath_print(&self.0, str.as_mut_ptr())
        });

        if status_code.is_good() {
            Ok(str.to_string())
        } else {
            Err(Error::new(status_code))
        }
    }
}

