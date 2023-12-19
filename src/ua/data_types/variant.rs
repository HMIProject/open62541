use std::ffi::c_void;

use open62541_sys::{UA_Variant_hasScalarType, UA_Variant_setScalarCopy};

use crate::data_type::DataType;

crate::data_type!(Variant, UA_Variant, UA_TYPES_VARIANT);

impl Variant {
    #[must_use]
    pub fn scalar<T: DataType>(&self) -> Option<T> {
        if !unsafe { UA_Variant_hasScalarType(self.as_ptr(), T::data_type()) } {
            return None;
        }

        unsafe { self.0.data.cast::<T::Inner>().as_ref() }.map(|value| T::from_ref(value))
    }

    #[must_use]
    pub fn with_scalar<T: DataType>(mut self, value: &T) -> Self {
        unsafe {
            UA_Variant_setScalarCopy(
                self.as_mut_ptr(),
                value.as_ptr().cast::<c_void>(),
                T::data_type(),
            );
        }
        self
    }
}
