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
