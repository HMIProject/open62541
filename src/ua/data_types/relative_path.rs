use std::{fmt, ptr, slice::SliceIndex, str::FromStr};

use open62541_sys::{UA_RelativePath_parse, UA_RelativePath_print};

use crate::{DataType as _, Error, ua};

crate::data_type!(RelativePath);

impl RelativePath {
    #[must_use]
    pub fn with_elements(mut self, elements: &[ua::RelativePathElement]) -> Self {
        let array = ua::Array::from_slice(elements);
        array.move_into_raw(&mut self.0.elementsSize, &mut self.0.elements);
        self
    }

    /// Returns a view of the elements that make up this relative path.
    ///
    /// Returns [`Some`] if the relative path element array is valid, otherwise [`None`].
    #[must_use]
    pub fn elements(&self) -> Option<&[ua::RelativePathElement]> {
        // SAFETY: Lifetime of the slice is implicitly bound to the lifetime of the reference to self.
        // Pointer validity is checked in `Array::slice_from_raw_parts()`.
        unsafe { ua::Array::slice_from_raw_parts(self.0.elementsSize, self.0.elements) }
    }

    /// Returns a mutable view of the elements that make up this relative path.
    ///
    /// Returns [`Some`] if the relative path element array is valid, otherwise [`None`].
    #[must_use]
    pub fn elements_mut(&mut self) -> Option<&mut [ua::RelativePathElement]> {
        // SAFETY: Lifetime of the slice is implicitly bound to the lifetime of the reference to self.
        // Pointer validity is checked in `Array::slice_from_raw_parts_mut()`.
        unsafe { ua::Array::slice_from_raw_parts_mut(self.0.elementsSize, self.0.elements) }
    }

    /// Attempts to access the element at `index`.
    ///
    /// Returns [`Some`] if the array is valid and `index` < [`RelativePath::len()`], otherwise [`None`]
    #[must_use]
    pub fn get<I>(&self, index: I) -> Option<&I::Output>
    where
        I: SliceIndex<[ua::RelativePathElement]>,
    {
        self.elements().and_then(|elements| elements.get(index))
    }

    /// Attempts to access the element at `index` mutably.
    ///
    /// Returns [`Some`] if the array is valid and `index` < [`RelativePath::len()`], otherwise [`None`]
    #[must_use]
    pub fn get_mut<I>(&mut self, index: I) -> Option<&mut I::Output>
    where
        I: SliceIndex<[ua::RelativePathElement]>,
    {
        self.elements_mut()
            .and_then(|elements| elements.get_mut(index))
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
        let mut parsed_path = Self::init();

        let result =
            unsafe { UA_RelativePath_parse(parsed_path.as_mut_ptr(), ptr::read(path.as_ptr())) };

        let status_code = ua::StatusCode::new(result);

        if status_code.is_good() {
            Ok(parsed_path)
        } else {
            Err(Error::new(status_code))
        }
    }

    pub fn iter(&self) -> impl ExactSizeIterator<Item = &ua::RelativePathElement> {
        self.elements().unwrap_or(&[]).iter()
    }

    pub fn iter_mut(&mut self) -> impl ExactSizeIterator<Item = &mut ua::RelativePathElement> {
        self.elements_mut().unwrap_or(&mut []).iter_mut()
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

        str.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::RelativePath;
    use crate::{DataType as _, ua};

    #[test]
    fn well_formed_path_parses_with_expected_output() {
        let parsed = RelativePath::parse("/Objects<#!HasChild>Server")
            .expect("well-formed `RelativePath` string should parse");

        assert_eq!(parsed.len(), 2);

        let elem = parsed.get(0).unwrap();
        let expected = ua::RelativePathElement::init()
            .with_include_subtypes(true)
            .with_is_inverse(false)
            .with_target_name(&ua::QualifiedName::new(0, "Objects"))
            .with_reference_type_id(&ua::NodeId::numeric(0, 33));

        assert_eq!(elem, &expected);

        let elem = parsed.get(1).unwrap();
        let expected = ua::RelativePathElement::init()
            .with_include_subtypes(false)
            .with_is_inverse(true)
            .with_target_name(&ua::QualifiedName::new(0, "Server"))
            .with_reference_type_id(&ua::NodeId::numeric(0, 34));

        assert_eq!(elem, &expected);
    }

    #[test]
    fn well_formed_path_survives_round_trip() {
        const GOOD_PATH: &str = "<!Aggregates>2:PLC1/2:GVL_MAIN";
        let parsed =
            RelativePath::parse(GOOD_PATH).expect("well-formed `RelativePath` string should parse");

        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed.to_string(), GOOD_PATH);
    }

    #[test]
    fn malformed_path_fails_to_parse() {
        const BAD_PATH: &str = "/!Bad!>/Path:1";
        let result = RelativePath::parse(BAD_PATH);
        assert!(result.is_err(), "parsing a bad path should fail");
    }
}
