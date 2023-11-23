use std::{fmt, ptr::NonNull};

use open62541_sys::{
    UA_Variant, UA_Variant_delete, UA_Variant_new, UA_print, UA_STATUSCODE_GOOD, UA_TYPES,
    UA_TYPES_VARIANT,
};

use crate::ua;

pub struct Variant(NonNull<UA_Variant>);

impl Variant {
    #[must_use]
    pub fn new() -> Option<Self> {
        // `UA_Variant_new` matches `UA_Variant_delete`.
        let ua_variant = NonNull::new(unsafe { UA_Variant_new() })?;

        Some(Self(ua_variant))
    }

    #[must_use]
    pub const fn as_ptr(&self) -> *const UA_Variant {
        self.0.as_ptr()
    }

    #[must_use]
    pub fn as_mut_ptr(&mut self) -> *mut UA_Variant {
        self.0.as_ptr()
    }
}

impl Drop for Variant {
    fn drop(&mut self) {
        // `UA_Variant_delete` matches `UA_Variant_new`.
        unsafe { UA_Variant_delete(self.as_mut_ptr()) }
    }
}

impl fmt::Debug for Variant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = ua::String::new();
        let data_type = unsafe { &UA_TYPES[UA_TYPES_VARIANT as usize] };

        let result = unsafe { UA_print(self.as_ptr().cast(), data_type, output.as_mut_ptr()) };

        if result != UA_STATUSCODE_GOOD {
            return f.write_str("Variant");
        }

        match output.as_str() {
            Some(str) => f.write_str(str),
            None => f.write_str("Variant"),
        }
    }
}
