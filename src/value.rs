use std::{ffi::c_void, ptr::NonNull};

use open62541_sys::UA_EMPTY_ARRAY_SENTINEL;

use crate::ua;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ValueType {
    /// Unsupported data type.
    ///
    /// This is a sentinel for an existing and set value that we do not support (yet). Depending on
    /// the circumstances, you might be able to use [`Variant::as_scalar()`] et al. instead.
    ///
    /// [`Variant::as_scalar()`]: ua::Variant::as_scalar
    Unsupported,
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
    Guid,           // Data type ns=0;i=14
    ByteString,     // Data type ns=0;i=15
    NodeId,         // Data type ns=0;i=17
    ExpandedNodeId, // Data type ns=0;i=18
    StatusCode,     // Data type ns=0;i=19
    QualifiedName,  // Data type ns=0;i=20
    LocalizedText,  // Data type ns=0;i=21
    Structure,      // Data type ns=0;i=22
    Enumeration,    // Data type ns=0;i=29
    Argument,       // Data type ns=0;i=296
}

impl ValueType {
    /// Gets value type from data type's node ID.
    ///
    /// This gets the [`ValueType`] corresponding to the given data type's node ID. If the node ID
    /// is not a known data type, [`ValueType::Unsupported`] is returned.
    #[must_use]
    pub fn from_data_type(node_id: &ua::NodeId) -> Self {
        macro_rules! check {
            ($numeric:ident, [$( $name:ident ),* $(,)?] $(,)?) => {
                if false {
                    unreachable!()
                }
                $(
                    else if $numeric == paste::paste!{ open62541_sys::[<UA_NS0ID_ $name:upper>] } {
                        ValueType::$name
                    }
                )*
                else {
                    ValueType::Unsupported
                }
            };
        }

        // We only support known data types in namespace 0.
        let Some(numeric) = node_id.as_ns0() else {
            return ValueType::Unsupported;
        };

        check!(
            numeric,
            [
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
                Guid,           // Data type ns=0;i=14
                ByteString,     // Data type ns=0;i=15
                NodeId,         // Data type ns=0;i=17
                ExpandedNodeId, // Data type ns=0;i=18
                StatusCode,     // Data type ns=0;i=19
                QualifiedName,  // Data type ns=0;i=20
                LocalizedText,  // Data type ns=0;i=21
                Structure,      // Data type ns=0;i=22
                Enumeration,    // Data type ns=0;i=29
                Argument,       // Data type ns=0;i=296
            ],
        )
    }
}

/// Value of [`ua::Variant`].
#[derive(Debug, Clone)]
pub enum VariantValue {
    Empty,
    Scalar(ScalarValue),
    // TODO: Add proper interface.
    #[expect(private_interfaces, reason = "pending refactor")]
    NonScalar(NonScalarValue),
}

/// Scalar value.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ScalarValue {
    /// Unsupported data type.
    ///
    /// This is a sentinel for an existing and set value that we do not support (yet). Depending on
    /// the circumstances, you might be able to use [`Variant::to_scalar()`] et al. instead.
    ///
    /// [`Variant::to_scalar()`]: ua::Variant::to_scalar
    Unsupported,
    // This mirrors the unwrapping in `Variant::to_value()`.
    Boolean(ua::Boolean),               // Data type ns=0;i=1
    SByte(ua::SByte),                   // Data type ns=0;i=2
    Byte(ua::Byte),                     // Data type ns=0;i=3
    Int16(ua::Int16),                   // Data type ns=0;i=4
    UInt16(ua::UInt16),                 // Data type ns=0;i=5
    Int32(ua::Int32),                   // Data type ns=0;i=6
    UInt32(ua::UInt32),                 // Data type ns=0;i=7
    Int64(ua::Int64),                   // Data type ns=0;i=8
    UInt64(ua::UInt64),                 // Data type ns=0;i=9
    Float(ua::Float),                   // Data type ns=0;i=10
    Double(ua::Double),                 // Data type ns=0;i=11
    String(ua::String),                 // Data type ns=0;i=12
    DateTime(ua::DateTime),             // Data type ns=0;i=13
    Guid(ua::Guid),                     // Data type ns=0;i=14
    ByteString(ua::ByteString),         // Data type ns=0;i=15
    NodeId(ua::NodeId),                 // Data type ns=0;i=17
    ExpandedNodeId(ua::ExpandedNodeId), // Data type ns=0;i=18
    StatusCode(ua::StatusCode),         // Data type ns=0;i=19
    QualifiedName(ua::QualifiedName),   // Data type ns=0;i=20
    LocalizedText(ua::LocalizedText),   // Data type ns=0;i=21
    Structure(ua::ExtensionObject),     // Data type ns=0;i=22
    Enumeration(ua::Enumeration),       // Data type ns=0;i=29
    Argument(ua::Argument),             // Data type ns=0;i=296
}

// TODO: Add proper interface.
#[derive(Debug, Clone)]
pub(crate) struct NonScalarValue;

/// Value that may be invalid or empty.
///
/// For some types (notably arrays and strings) OPC UA defines different states: an empty state and
/// an invalid state, in addition to the regular valid/non-empty state.
// TODO: Think about making this public.
#[derive(Debug, Clone)]
pub(crate) enum ArrayValue<T> {
    Invalid,
    Empty,
    Valid(NonNull<T>),
}

impl<T> ArrayValue<T> {
    /// Creates appropriate [`ArrayValue`].
    ///
    /// This checks for different states (null pointer, sentinel value) and returns the appropriate
    /// value from [`ArrayValue`].
    pub(crate) fn from_ptr(data: *mut T) -> Self {
        // Check for sentinel value first. We must not treat it as valid pointer below.
        if data.cast_const().cast::<c_void>() == unsafe { UA_EMPTY_ARRAY_SENTINEL } {
            return ArrayValue::Empty;
        }

        // Null pointers are regarded as "invalid" data by `open62541`.
        match NonNull::new(data) {
            Some(data) => ArrayValue::Valid(data),
            None => ArrayValue::Invalid,
        }
    }
}
