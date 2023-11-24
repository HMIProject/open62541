use std::{fmt, mem, ptr};

use open62541_sys::{
    UA_Variant, UA_Variant_clear, UA_Variant_copy, UA_Variant_init, UA_print, UA_STATUSCODE_GOOD,
    UA_TYPES, UA_TYPES_VARIANT,
};

use crate::ua;

pub struct Variant(UA_Variant);

impl Variant {
    #[allow(dead_code)]
    #[must_use]
    pub fn new() -> Self {
        let mut variant = unsafe { mem::MaybeUninit::<UA_Variant>::zeroed().assume_init() };
        unsafe { UA_Variant_init(ptr::addr_of_mut!(variant)) }
        Self(variant)
    }

    /// Copies value from `src`.
    #[allow(dead_code)]
    pub(crate) fn from(src: &UA_Variant) -> Self {
        let mut dst = Self::new();
        let result = unsafe { UA_Variant_copy(src, dst.as_mut_ptr()) };
        assert_eq!(result, UA_STATUSCODE_GOOD);
        dst
    }

    /// Takes ownership of `src`.
    #[allow(dead_code)]
    pub(crate) fn from_inner(src: UA_Variant) -> Self {
        Variant(src)
    }

    #[allow(dead_code)]
    pub(crate) const fn as_ref(&self) -> &UA_Variant {
        &self.0
    }

    #[allow(dead_code)]
    pub(crate) fn as_mut(&mut self) -> &mut UA_Variant {
        &mut self.0
    }

    #[allow(dead_code)]
    pub(crate) const fn as_ptr(&self) -> *const UA_Variant {
        ptr::addr_of!(self.0)
    }

    #[allow(dead_code)]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut UA_Variant {
        ptr::addr_of_mut!(self.0)
    }

    #[allow(dead_code)]
    pub(crate) fn into_inner(self) -> UA_Variant {
        let variant = self.0;
        mem::forget(self);
        variant
    }
}

impl Drop for Variant {
    fn drop(&mut self) {
        unsafe { UA_Variant_clear(self.as_mut_ptr()) }
    }
}

impl Default for Variant {
    fn default() -> Self {
        Self::new()
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
