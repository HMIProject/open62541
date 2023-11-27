use std::{slice, str};

use crate::ua;

ua::data_type!(String, UA_String, UA_TYPES_STRING);

impl String {
    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        let slice = unsafe { slice::from_raw_parts(self.0.data, self.0.length) };
        str::from_utf8(slice).ok()
    }
}
