use std::{fmt, ptr::NonNull};

use log::debug;
use open62541_sys::{UA_Variant, UA_Variant_clear, UA_Variant_delete, UA_Variant_new};

pub struct Variant(NonNull<UA_Variant>);

impl Variant {
    pub fn new() -> Option<Self> {
        debug!("Creating UA_Variant");

        let variant = NonNull::new(unsafe { UA_Variant_new() })?;

        unsafe { UA_Variant_clear(variant.as_ptr()) };

        Some(Variant(variant))
    }

    pub const fn as_ptr(&self) -> *mut UA_Variant {
        self.0.as_ptr()
    }
}

impl Drop for Variant {
    fn drop(&mut self) {
        debug!("Dropping UA_Variant");

        unsafe { UA_Variant_delete(self.0.as_ptr()) }
    }
}

impl fmt::Debug for Variant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Variant")
    }
}
