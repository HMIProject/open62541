use std::{ffi::c_void, ptr::NonNull};

use open62541_sys::UA_EMPTY_ARRAY_SENTINEL_;

use crate::ua;

#[derive(Debug, Clone)]
pub enum VariantValue {
    Empty,
    Scalar(ScalarValue),
    // TODO: Support arrays.
}

#[derive(Debug, Clone)]
pub enum ScalarValue {
    Unknown,
    Boolean(ua::Boolean),   // Data type ns=0;i=1
    SByte(ua::SByte),       // Data type ns=0;i=2
    Byte(ua::Byte),         // Data type ns=0;i=3
    Int16(ua::Int16),       // Data type ns=0;i=4
    UInt16(ua::UInt16),     // Data type ns=0;i=5
    Int32(ua::Int32),       // Data type ns=0;i=6
    UInt32(ua::UInt32),     // Data type ns=0;i=7
    Int64(ua::Int64),       // Data type ns=0;i=8
    UInt64(ua::UInt64),     // Data type ns=0;i=9
    Float(ua::Float),       // Data type ns=0;i=10
    Double(ua::Double),     // Data type ns=0;i=11
    String(ua::String),     // Data type ns=0;i=12
    DateTime(ua::DateTime), // Data type ns=0;i=13
}

/// Value that may be invalid or empty.
///
/// For some types (notably arrays and strings) OPC UA defines different states: an empty state and
/// an invalid state, in addition to the regular valid state.
#[derive(Debug, Clone)]
pub enum MaybeValue<T> {
    Invalid,
    Empty,
    Valid(T),
}

pub type PointerValue<T> = MaybeValue<NonNull<T>>;

impl<T> PointerValue<T> {
    /// Creates wrapped pointer.
    ///
    /// This checks for different states (null pointer, sentinel value) and returns the appropriate
    /// value from [`MaybeValue`].
    pub fn from_raw(data: *mut T) -> Self {
        // Check for sentinel value first. We must not treat it as valid pointer below.
        if data.cast_const().cast::<c_void>() == unsafe { UA_EMPTY_ARRAY_SENTINEL_ } {
            return PointerValue::Empty;
        }

        // Null pointers are regarded as "invalid" data by `open62541`.
        match NonNull::new(data) {
            Some(data) => PointerValue::Valid(data),
            None => PointerValue::Invalid,
        }
    }
}
