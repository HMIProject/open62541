use std::ffi::c_void;

use open62541_sys::{
    UA_Variant_clear, UA_Variant_hasArrayType, UA_Variant_hasScalarType, UA_Variant_isEmpty,
    UA_Variant_isScalar, UA_Variant_setArray, UA_Variant_setScalar, UA_Variant_setScalarCopy,
};

use crate::{ua, DataType, NonScalarValue, ScalarValue, ValueType, VariantValue};

crate::data_type!(Variant);

impl Variant {
    /// Creates variant from scalar.
    #[must_use]
    pub fn scalar<T: DataType>(value: T) -> Self {
        let mut variant = Self::init();
        // This gives up ownership of the scalar, then moves it into the variant which becomes the
        // new owner.
        let ptr = value.leak_into_raw();
        unsafe {
            UA_Variant_setScalar(variant.as_mut_ptr(), ptr.cast::<c_void>(), T::data_type());
        }
        variant
    }

    /// Creates variant from array.
    #[must_use]
    pub fn array<T: DataType>(value: ua::Array<T>) -> Self {
        let mut variant = Self::init();
        // This gives up ownership of the array, then moves it into the variant which becomes the
        // new owner.
        let (size, ptr) = value.into_raw_parts();
        unsafe {
            UA_Variant_setArray(
                variant.as_mut_ptr(),
                ptr.cast::<c_void>(),
                size,
                T::data_type(),
            );
        }
        variant
    }

    #[must_use]
    pub fn with_scalar<T: DataType>(mut self, value: &T) -> Self {
        // The call to `UA_Variant_setScalarCopy()` does not free held memory which would lead to a
        // memory leak. We must clear the variant manually to handle the case where `with_scalar()`
        // is called multiple times on the same `Variant`.
        unsafe {
            UA_Variant_clear(self.as_mut_ptr());
            UA_Variant_setScalarCopy(
                self.as_mut_ptr(),
                value.as_ptr().cast::<c_void>(),
                T::data_type(),
            );
        }
        self
    }

    /// Gets data type's node ID.
    ///
    /// Returns `None` when the variant is empty.
    #[must_use]
    pub fn type_id(&self) -> Option<&ua::NodeId> {
        let data_type = unsafe { self.0.type_.as_ref() };
        data_type.map(|data_type| ua::NodeId::raw_ref(&data_type.typeId))
    }

    /// Gets value type.
    ///
    /// Returns `None` when the variant is empty.
    #[must_use]
    pub fn value_type(&self) -> Option<ValueType> {
        self.type_id().map(ValueType::from_data_type)
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
    pub fn to_array<T: DataType>(&self) -> Option<ua::Array<T>> {
        if !unsafe { UA_Variant_hasArrayType(self.as_ptr(), T::data_type()) } {
            // Special case: open62541 automatically converts arrays of extension objects into the
            // contained data type (as of version 1.4). This only works for non-empty arrays since
            // the element type needs to be known.
            //
            // To make handling such arrays easier in user code, we allow _coercion_ of such empty
            // arrays into any data type.
            let is_empty_structured_array = self.0.arrayLength == 0
                && unsafe {
                    UA_Variant_hasArrayType(self.as_ptr(), ua::ExtensionObject::data_type())
                };
            if !is_empty_structured_array {
                return None;
            }
            // Fall through to let `ua::Array::from_raw_parts()` handle the distinction between an
            // empty and an invalid array (where `self.0.data` is the sentinel value or null).
        }
        ua::Array::from_raw_parts(self.0.data.cast::<T::Inner>(), self.0.arrayLength)
    }

    #[must_use]
    pub fn to_value(&self) -> VariantValue {
        if self.is_empty() {
            return VariantValue::Empty;
        }

        if !self.is_scalar() {
            // TODO: Handle non-scalar (array) values.
            return VariantValue::NonScalar(NonScalarValue);
        }

        macro_rules! check {
            ($( $name:ident ),* $(,)?) => {
                $(
                    if let Some(value) = self.to_scalar::<ua::$name>() {
                        return VariantValue::Scalar(ScalarValue::$name(value));
                    }
                )*
            };
        }

        check!(
            Boolean,        // Data type ns=0;i=1
            SByte,          // Data type ns=0;i=2
            Byte,           // Data type ns=0;i=3
            Int16,          // Data type ns=0;i=4
            UInt16,         // Data type ns=0;i=5
            Int32,          // Data type ns=0;i=6
            UInt32,         // Data type ns=0;i=7
            Int64,          // Data type ns=0;i=8
            UInt64,         // Data type ns=0;i=9
            Float,          // Data type ns=0;i=10
            Double,         // Data type ns=0;i=11
            String,         // Data type ns=0;i=12
            DateTime,       // Data type ns=0;i=13
            ByteString,     // Data type ns=0;i=15
            NodeId,         // Data type ns=0;i=17
            ExpandedNodeId, // Data type ns=0;i=18
            StatusCode,     // Data type ns=0;i=19
            QualifiedName,  // Data type ns=0;i=20
            LocalizedText,  // Data type ns=0;i=21
            Argument,       // Data type ns=0;i=296
        );

        VariantValue::Scalar(ScalarValue::Unsupported)
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
        macro_rules! serialize {
            ($self:ident, $serializer:ident, [$( $( #[cfg($cfg: meta)] )? $name:ident ),* $(,)?] $(,)?) => {
                $(
                    $( #[cfg($cfg)] )?
                    if let Some(value) = self.as_scalar::<crate::ua::$name>() {
                        return <crate::ua::$name as serde::Serialize>::serialize(value, serializer);
                    }

                    $( #[cfg($cfg)] )?
                    if let Some(value) = self.to_array::<crate::ua::$name>() {
                        // TODO: Avoid `to_array()`, borrow `value` from `self` instead of copying.
                        return <ua::Array<crate::ua::$name> as serde::Serialize>::serialize(&value, serializer);
                    }
                )*
            };
        }

        serialize!(
            self,
            serializer,
            [
                Boolean, // Data type ns=0;i=1
                SByte,   // Data type ns=0;i=2
                Byte,    // Data type ns=0;i=3
                Int16,   // Data type ns=0;i=4
                UInt16,  // Data type ns=0;i=5
                Int32,   // Data type ns=0;i=6
                UInt32,  // Data type ns=0;i=7
                Int64,   // Data type ns=0;i=8
                UInt64,  // Data type ns=0;i=9
                Float,   // Data type ns=0;i=10
                Double,  // Data type ns=0;i=11
                String,  // Data type ns=0;i=12
                #[cfg(feature = "time")]
                DateTime, // Data type ns=0;i=13
                ByteString, // Data type ns=0;i=15
                NodeId,  // Data type ns=0;i=17
            ],
        );

        // The following types are deliberately missing from the list abvove because we don't have a
        // good serialization for them:
        //
        // - ExpandedNodeId, // Data type ns=0;i=18
        // - StatusCode,     // Data type ns=0;i=19
        // - QualifiedName,  // Data type ns=0;i=20
        // - LocalizedText,  // Data type ns=0;i=21
        // - Argument,       // Data type ns=0;i=296

        Err(serde::ser::Error::custom("non-primitive value in Variant"))
    }
}

#[cfg(test)]
mod tests {
    use open62541_sys::{
        UA_NS0ID_BOOLEAN, UA_NS0ID_BYTE, UA_NS0ID_INT16, UA_NS0ID_INT64, UA_NS0ID_UINT32,
    };

    use crate::{ua, DataType as _, ValueType};

    #[test]
    fn type_empty() {
        let ua_variant = ua::Variant::init();
        let type_id = ua_variant.type_id();
        assert_eq!(type_id, None);
        let value_type = ua_variant.value_type();
        assert_eq!(value_type, None);
    }

    #[test]
    fn type_boolean() {
        let ua_bool = ua::Boolean::new(true);
        let ua_variant = ua::Variant::scalar(ua_bool);
        let type_id = ua_variant.type_id();
        assert_eq!(type_id, Some(&ua::NodeId::ns0(UA_NS0ID_BOOLEAN)));
        let value_type = ua_variant.value_type();
        assert_eq!(value_type, Some(ValueType::Boolean));
    }

    #[test]
    fn type_int() {
        // Byte
        let ua_byte = ua::Byte::new(42);
        let ua_variant = ua::Variant::scalar(ua_byte);
        let type_id = ua_variant.type_id();
        assert_eq!(type_id, Some(&ua::NodeId::ns0(UA_NS0ID_BYTE)));
        let value_type = ua_variant.value_type();
        assert_eq!(value_type, Some(ValueType::Byte));

        // Int16
        let ua_int16 = ua::Int16::new(-12345);
        let ua_variant = ua::Variant::scalar(ua_int16);
        let type_id = ua_variant.type_id();
        assert_eq!(type_id, Some(&ua::NodeId::ns0(UA_NS0ID_INT16)));
        let value_type = ua_variant.value_type();
        assert_eq!(value_type, Some(ValueType::Int16));

        // UInt32
        let ua_uint32 = ua::UInt32::new(123_456_789);
        let ua_variant = ua::Variant::scalar(ua_uint32);
        let type_id = ua_variant.type_id();
        assert_eq!(type_id, Some(&ua::NodeId::ns0(UA_NS0ID_UINT32)));
        let value_type = ua_variant.value_type();
        assert_eq!(value_type, Some(ValueType::UInt32));

        // Int64
        let ua_int64 = ua::Int64::new(-7_077_926_753_204_279_296);
        let ua_variant = ua::Variant::scalar(ua_int64);
        let type_id = ua_variant.type_id();
        assert_eq!(type_id, Some(&ua::NodeId::ns0(UA_NS0ID_INT64)));
        let value_type = ua_variant.value_type();
        assert_eq!(value_type, Some(ValueType::Int64));
    }

    #[test]
    fn array_ops() {
        let ua_array = ua::Array::from_slice(&[1, 2, 3].map(ua::Byte::new));
        let ua_variant = ua::Variant::array(ua_array);
        let type_id = ua_variant.type_id();
        assert_eq!(type_id, Some(&ua::NodeId::ns0(UA_NS0ID_BYTE)));
        let value_type = ua_variant.value_type();
        assert_eq!(value_type, Some(ValueType::Byte));

        assert!(ua_variant.to_array::<ua::String>().is_none());
        let ua_array: ua::Array<ua::Byte> = ua_variant.to_array().unwrap();
        assert_eq!(
            vec![ua::Byte::new(1), ua::Byte::new(2), ua::Byte::new(3)],
            ua_array.into_vec(),
        );
    }

    #[cfg(feature = "serde")]
    mod serde {
        use crate::ua;

        #[test]
        fn serialize_bool() {
            // Value `true`
            let ua_bool = ua::Boolean::new(true);
            let ua_variant = ua::Variant::scalar(ua_bool);
            let json = serde_json::to_string(&ua_variant).unwrap();
            assert_eq!("true", json);

            // Value `false`
            let ua_bool = ua::Boolean::new(false);
            let ua_variant = ua::Variant::scalar(ua_bool);
            let json = serde_json::to_string(&ua_variant).unwrap();
            assert_eq!("false", json);
        }

        #[test]
        fn serialize_int() {
            // Byte (unsigned)
            let ua_byte = ua::Byte::new(42);
            let ua_variant = ua::Variant::scalar(ua_byte);
            let json = serde_json::to_string(&ua_variant).unwrap();
            assert_eq!("42", json);

            // Int16 (signed)
            let ua_int16 = ua::Int16::new(-12345);
            let ua_variant = ua::Variant::scalar(ua_int16);
            let json = serde_json::to_string(&ua_variant).unwrap();
            assert_eq!("-12345", json);

            // UInt32 (unsigned)
            let ua_uint32 = ua::UInt32::new(123_456_789);
            let ua_variant = ua::Variant::scalar(ua_uint32);
            let json = serde_json::to_string(&ua_variant).unwrap();
            assert_eq!("123456789", json);

            // Int64 (signed)
            let ua_int64 = ua::Int64::new(-7_077_926_753_204_279_296);
            let ua_variant = ua::Variant::scalar(ua_int64);
            let json = serde_json::to_string(&ua_variant).unwrap();
            assert_eq!("-7077926753204279296", json);
        }

        #[test]
        fn serialize_float() {
            // Float
            let ua_float = ua::Float::new(123.4567);
            let ua_variant = ua::Variant::scalar(ua_float);
            let json = serde_json::to_string(&ua_variant).unwrap();
            assert_eq!("123.4567", json);

            // Double
            let ua_double = ua::Double::new(-98_765_432.1);
            let ua_variant = ua::Variant::scalar(ua_double);
            let json = serde_json::to_string(&ua_variant).unwrap();
            assert_eq!("-98765432.1", json);
        }

        #[test]
        fn serialize_string() {
            // Empty string
            let ua_string = ua::String::new("").unwrap();
            let ua_variant = ua::Variant::scalar(ua_string);
            let json = serde_json::to_string(&ua_variant).unwrap();
            assert_eq!(r#""""#, json);

            // Short string
            let ua_string = ua::String::new("lorem ipsum").unwrap();
            let ua_variant = ua::Variant::scalar(ua_string);
            let json = serde_json::to_string(&ua_variant).unwrap();
            assert_eq!(r#""lorem ipsum""#, json);

            // Special characters
            let ua_string = ua::String::new(r#"a'b"c{dẞe"#).unwrap();
            let ua_variant = ua::Variant::scalar(ua_string);
            let json = serde_json::to_string(&ua_variant).unwrap();
            assert_eq!(r#""a'b\"c{dẞe""#, json);
        }

        #[cfg(feature = "time")]
        #[test]
        fn serialize_datetime() {
            // Minute precision
            let datetime = time::macros::datetime!(2024-02-09 16:48 UTC);
            let ua_datetime = ua::DateTime::try_from(datetime).unwrap();
            let ua_variant = ua::Variant::scalar(ua_datetime);
            let json = serde_json::to_string(&ua_variant).unwrap();
            assert_eq!(r#""2024-02-09T16:48:00Z""#, json);

            // Microsecond precision
            let datetime = time::macros::datetime!(2024-02-09 16:48:52.123456 UTC);
            let ua_datetime = ua::DateTime::try_from(datetime).unwrap();
            let ua_variant = ua::Variant::scalar(ua_datetime);
            let json = serde_json::to_string(&ua_variant).unwrap();
            assert_eq!(r#""2024-02-09T16:48:52.123456Z""#, json);
        }

        #[test]
        fn serialize_array() {
            let ua_array = ua::Array::from_slice(&[1, 2, 3].map(ua::Byte::new));
            let ua_variant = ua::Variant::array(ua_array);
            let json = serde_json::to_string(&ua_variant).unwrap();
            assert_eq!("[1,2,3]", json);

            let ua_array = ua::Array::from_slice(&[
                ua::String::new("lorem").unwrap(),
                ua::String::new(r#"ip"sum"#).unwrap(),
            ]);
            let ua_variant = ua::Variant::array(ua_array);
            let json = serde_json::to_string(&ua_variant).unwrap();
            assert_eq!(r#"["lorem","ip\"sum"]"#, json);
        }
    }
}
