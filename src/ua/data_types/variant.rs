use std::ffi::c_void;

use open62541_sys::{UA_Variant_hasScalarType, UA_Variant_setScalarCopy};

use crate::data_type::DataType;

crate::data_type!(Variant, UA_Variant, UA_TYPES_VARIANT);

impl Variant {
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

    #[must_use]
    pub fn scalar<T: DataType>(&self) -> Option<T> {
        if !unsafe { UA_Variant_hasScalarType(self.as_ptr(), T::data_type()) } {
            return None;
        }
        unsafe { self.0.data.cast::<T::Inner>().as_ref() }.map(|value| T::from_ref(value))
    }

    #[cfg(feature = "serde")]
    #[must_use]
    pub fn json(&self) -> Option<serde_json::Value> {
        serde_json::to_value(self).ok()
    }
}

#[cfg(feature = "serde")]
mod serde {
    use serde::ser;

    use super::Variant;

    impl serde::Serialize for Variant {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            macro_rules! serialize {
                ($self:ident, $serializer:ident, [ $( ($name:ident, $type:ty) ),+ $(,)? ]) => {
                    $(
                        if let Some(value) = $self.scalar::<crate::ua::$name>() {
                            paste::paste! {
                                return $serializer.[<serialize_ $type>](value.into_inner());
                            }
                        }
                    )+
                };
            }

            serialize!(
                self,
                serializer,
                [
                    (Boolean, bool),
                    (SByte, i8),
                    (Byte, u8),
                    (Int16, i16),
                    (UInt16, u16),
                    (Int32, i32),
                    (UInt32, u32),
                    (Int64, i64),
                    (UInt64, u64),
                    (Float, f32),
                    (Double, f64),
                ]
            );

            Err(ser::Error::custom("non-primitive value in Variant"))
        }
    }
}
