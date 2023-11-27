use std::{ffi::c_void, fmt};

use open62541_sys::{UA_print, UA_STATUSCODE_GOOD, UA_TYPES, UA_TYPES_VARIANT};

use crate::{ua, DataType};

ua::data_type!(Variant, UA_Variant, UA_TYPES_VARIANT);

impl fmt::Debug for Variant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = ua::String::default();
        let data_type = unsafe { &UA_TYPES[UA_TYPES_VARIANT as usize] };

        let result = unsafe {
            UA_print(
                self.as_ptr().cast::<c_void>(),
                data_type,
                output.as_mut_ptr(),
            )
        };

        if result != UA_STATUSCODE_GOOD {
            return f.write_str("Variant");
        }

        match output.as_str() {
            Some(str) => f.write_str(str),
            None => f.write_str("Variant"),
        }
    }
}
