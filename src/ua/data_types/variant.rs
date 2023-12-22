use std::ffi::c_void;

use open62541_sys::{
    UA_Variant_hasScalarType, UA_Variant_isEmpty, UA_Variant_isScalar, UA_Variant_setScalarCopy,
};

use crate::{data_type::DataType, ua};

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
    pub fn is_empty(&self) -> bool {
        unsafe { UA_Variant_isEmpty(self.as_ptr()) }
    }

    #[must_use]
    pub fn is_scalar(&self) -> bool {
        unsafe { UA_Variant_isScalar(self.as_ptr()) }
    }

    // TODO
    // #[must_use]
    // pub fn as_scalar<T: DataType>(&self) -> Option<&T> {
    //     if !unsafe { UA_Variant_hasScalarType(self.as_ptr(), T::data_type()) } {
    //         return None;
    //     }
    //     unsafe { self.0.data.cast::<T::Inner>().as_ref() }.map(|value| T::get_ref(value))
    // }

    #[must_use]
    pub fn to_scalar<T: DataType>(&self) -> Option<T> {
        if !unsafe { UA_Variant_hasScalarType(self.as_ptr(), T::data_type()) } {
            return None;
        }
        unsafe { self.0.data.cast::<T::Inner>().as_ref() }.map(|value| T::clone_raw(value))
    }

    #[must_use]
    pub fn into_value(self) -> ua::VariantValue {
        if self.is_empty() {
            return ua::VariantValue::Empty;
        }

        if !self.is_scalar() {
            todo!("should handle non-scalar value");
        }

        macro_rules! check {
            ([ $( $name:ident ),* $(,)? ]) => {
                $(
                    if let Some(value) = self.to_scalar::<ua::$name>() {
                        return ua::VariantValue::Scalar(ua::ScalarValue::$name(value));
                    }
                )*
            };
        }

        check!([
            Boolean,  // Data type ns=0;i=1
            SByte,    // Data type ns=0;i=2
            Byte,     // Data type ns=0;i=3
            Int16,    // Data type ns=0;i=4
            UInt16,   // Data type ns=0;i=5
            Int32,    // Data type ns=0;i=6
            UInt32,   // Data type ns=0;i=7
            Int64,    // Data type ns=0;i=8
            UInt64,   // Data type ns=0;i=9
            Float,    // Data type ns=0;i=10
            Double,   // Data type ns=0;i=11
            String,   // Data type ns=0;i=12
            DateTime, // Data type ns=0;i=13
        ]);

        ua::VariantValue::Scalar(ua::ScalarValue::Unknown)
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

    use crate::{ua, DataType as _};

    use super::Variant;

    impl serde::Serialize for Variant {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            macro_rules! serialize {
                ($self:ident, $serializer:ident, [ $( ($name:ident, $type:ty) ),* $(,)? ]) => {
                    $(
                        if let Some(value) = $self.to_scalar::<crate::ua::$name>() {
                            paste::paste! {
                                return $serializer.[<serialize_ $type>](value.into_raw());
                            }
                        }
                    )*
                };
            }

            serialize!(
                self,
                serializer,
                [
                    (Boolean, bool), // Data type ns=0;i=1
                    (SByte, i8),     // Data type ns=0;i=2
                    (Byte, u8),      // Data type ns=0;i=3
                    (Int16, i16),    // Data type ns=0;i=4
                    (UInt16, u16),   // Data type ns=0;i=5
                    (Int32, i32),    // Data type ns=0;i=6
                    (UInt32, u32),   // Data type ns=0;i=7
                    (Int64, i64),    // Data type ns=0;i=8
                    (UInt64, u64),   // Data type ns=0;i=9
                    (Float, f32),    // Data type ns=0;i=10
                    (Double, f64),   // Data type ns=0;i=11
                ]
            );

            // Data type ns=0;i=12
            if let Some(value) = self
                .to_scalar::<ua::String>()
                .as_ref()
                .and_then(|value| value.as_str())
            {
                return serializer.serialize_str(value);
            }

            // Data type ns=0;i=13
            #[cfg(feature = "time")]
            if let Some(value) = self
                .to_scalar::<ua::DateTime>()
                .and_then(|value| value.as_datetime())
            {
                return value.serialize(serializer);
            }

            Err(ser::Error::custom("non-primitive value in Variant"))
        }
    }
}
