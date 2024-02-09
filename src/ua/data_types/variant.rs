use std::ffi::c_void;

use open62541_sys::{
    UA_Variant_hasScalarType, UA_Variant_isEmpty, UA_Variant_isScalar, UA_Variant_setScalarCopy,
};

use crate::{data_type::DataType, ua};

crate::data_type!(Variant);

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

    #[must_use]
    pub fn as_scalar<T: DataType>(&self) -> Option<&T> {
        if !unsafe { UA_Variant_hasScalarType(self.as_ptr(), T::data_type()) } {
            return None;
        }
        unsafe { self.0.data.cast::<T::Inner>().as_ref() }.map(|value| T::raw_ref(value))
    }

    #[must_use]
    pub fn to_scalar<T: DataType>(&self) -> Option<T> {
        if !unsafe { UA_Variant_hasScalarType(self.as_ptr(), T::data_type()) } {
            return None;
        }
        unsafe { self.0.data.cast::<T::Inner>().as_ref() }.map(|value| T::clone_raw(value))
    }

    #[must_use]
    pub fn to_value(&self) -> ua::VariantValue {
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
impl serde::Serialize for Variant {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        macro_rules! serialize_raw {
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

        serialize_raw!(
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

        macro_rules! serialize {
            ($self:ident, $serializer:ident, [ $( $( #[cfg($cfg: meta)] )? $name:ident ),* $(,)? ]) => {
                $(
                    $( #[cfg($cfg)] )?
                    if let Some(value) = self.as_scalar::<crate::ua::$name>() {
                        return <crate::ua::$name as serde::Serialize>::serialize(value, serializer);
                    }
                )*
            };
        }

        serialize!(
            self,
            serializer,
            [
                String, // Data type ns=0;i=12
                #[cfg(feature = "time")]
                DateTime, // Data type ns=0;i=13
            ]
        );

        Err(serde::ser::Error::custom("non-primitive value in Variant"))
    }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    use crate::{ua, DataType as _};

    #[test]
    fn serialize_bool() {
        // Value `true`
        let ua_bool = ua::Boolean::new(true);
        let ua_variant = ua::Variant::init().with_scalar(&ua_bool);
        let json = serde_json::to_string(&ua_variant).unwrap();
        assert_eq!("true", json);

        // Value `false`
        let ua_bool = ua::Boolean::new(false);
        let ua_variant = ua::Variant::init().with_scalar(&ua_bool);
        let json = serde_json::to_string(&ua_variant).unwrap();
        assert_eq!("false", json);
    }

    #[test]
    fn serialize_int() {
        // Byte (unsigned)
        let ua_byte = ua::Byte::new(42);
        let ua_variant = ua::Variant::init().with_scalar(&ua_byte);
        let json = serde_json::to_string(&ua_variant).unwrap();
        assert_eq!("42", json);

        // Int16 (signed)
        let ua_int16 = ua::Int16::new(-12345);
        let ua_variant = ua::Variant::init().with_scalar(&ua_int16);
        let json = serde_json::to_string(&ua_variant).unwrap();
        assert_eq!("-12345", json);

        // UInt32 (unsigned)
        let ua_uint32 = ua::UInt32::new(123_456_789);
        let ua_variant = ua::Variant::init().with_scalar(&ua_uint32);
        let json = serde_json::to_string(&ua_variant).unwrap();
        assert_eq!("123456789", json);

        // Int64 (signed)
        let ua_int64 = ua::Int64::new(-7_077_926_753_204_279_296);
        let ua_variant = ua::Variant::init().with_scalar(&ua_int64);
        let json = serde_json::to_string(&ua_variant).unwrap();
        assert_eq!("-7077926753204279296", json);
    }

    #[test]
    fn serialize_float() {
        // Float
        let ua_float = ua::Float::new(123.4567);
        let ua_variant = ua::Variant::init().with_scalar(&ua_float);
        let json = serde_json::to_string(&ua_variant).unwrap();
        assert_eq!("123.4567", json);

        // Double
        let ua_double = ua::Double::new(-98_765_432.1);
        let ua_variant = ua::Variant::init().with_scalar(&ua_double);
        let json = serde_json::to_string(&ua_variant).unwrap();
        assert_eq!("-98765432.1", json);
    }

    #[test]
    fn serialize_string() {
        // Empty string
        let ua_string = ua::String::from_str("").unwrap();
        let ua_variant = ua::Variant::init().with_scalar(&ua_string);
        let json = serde_json::to_string(&ua_variant).unwrap();
        assert_eq!(r#""""#, json);

        // Short string
        let ua_string = ua::String::from_str("lorem ipsum").unwrap();
        let ua_variant = ua::Variant::init().with_scalar(&ua_string);
        let json = serde_json::to_string(&ua_variant).unwrap();
        assert_eq!(r#""lorem ipsum""#, json);

        // Special characters
        let ua_string = ua::String::from_str(r#"a'b"c{dẞe"#).unwrap();
        let ua_variant = ua::Variant::init().with_scalar(&ua_string);
        let json = serde_json::to_string(&ua_variant).unwrap();
        assert_eq!(r#""a'b\"c{dẞe""#, json);
    }

    #[cfg(feature = "time")]
    #[test]
    fn serialize_datetime() {
        // Minute precision
        let datetime = time::macros::datetime!(2024-02-09 16:48 UTC);
        let ua_datetime = ua::DateTime::try_from(datetime).unwrap();
        let ua_variant = ua::Variant::init().with_scalar(&ua_datetime);
        let json = serde_json::to_string(&ua_variant).unwrap();
        assert_eq!(r#""2024-02-09T16:48:00Z""#, json);

        // Microsecond precision
        let datetime = time::macros::datetime!(2024-02-09 16:48:52.123456 UTC);
        let ua_datetime = ua::DateTime::try_from(datetime).unwrap();
        let ua_variant = ua::Variant::init().with_scalar(&ua_datetime);
        let json = serde_json::to_string(&ua_variant).unwrap();
        assert_eq!(r#""2024-02-09T16:48:52.123456Z""#, json);
    }
}
