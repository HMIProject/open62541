use std::{borrow::Cow, slice, str, string};

crate::data_type!(String, UA_String, UA_TYPES_STRING);

impl String {
    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        let slice = unsafe { slice::from_raw_parts(self.0.data, self.0.length) };
        str::from_utf8(slice).ok()
    }

    #[must_use]
    pub fn to_string(&self) -> Cow<'_, str> {
        let slice = unsafe { slice::from_raw_parts(self.0.data, self.0.length) };
        string::String::from_utf8_lossy(slice)
    }
}
