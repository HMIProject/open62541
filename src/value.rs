use std::{ffi::c_void, ptr::NonNull};

use open62541_sys::UA_EMPTY_ARRAY_SENTINEL;

use crate::ua;

#[derive(Debug, Clone)]
#[allow(clippy::module_name_repetitions)]
pub enum VariantValue {
    Empty,
    Scalar(ScalarValue),
    // TODO: Support arrays.
}

#[derive(Debug, Clone)]
#[allow(clippy::module_name_repetitions)]
pub enum ScalarValue {
    Unknown,
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
    ByteString(ua::ByteString),         // Data type ns=0;i=15
    NodeId(ua::NodeId),                 // Data type ns=0;i=17
    ExpandedNodeId(ua::ExpandedNodeId), // Data type ns=0;i=18
    StatusCode(ua::StatusCode),         // Data type ns=0;i=19
    QualifiedName(ua::QualifiedName),   // Data type ns=0;i=20
    LocalizedText(ua::LocalizedText),   // Data type ns=0;i=21
    Argument(ua::Argument),             // Data type ns=0;i=296
}

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
    pub fn from_ptr(data: *mut T) -> Self {
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
