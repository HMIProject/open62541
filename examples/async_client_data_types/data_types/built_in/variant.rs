use super::{
    Boolean, Byte, ByteString, DataValue, DateTime, DiagnosticInfo, Double, ExpandedNodeId,
    ExtensionObject, Float, Guid, Int16, Int32, Int64, LocalizedText, NodeId, QualifiedName, SByte,
    StatusCode, String, UInt16, UInt32, UInt64, XmlElement,
};

// [Part 6: 5.1.9 Variant](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.9)
// [Part 6: 5.1.11 Null, Empty and Zero-Length Arrays](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.11)
pub enum Variant {
    Null,
    Scalar(VariantScalar),
    Array(Option<VariantArray>, Option<Vec<i32>>),
}

impl Variant {
    pub fn null() -> Self {
        Self::Null
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    pub fn is_scalar(&self) -> bool {
        matches!(self, Self::Scalar(_))
    }

    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(..))
    }
}

pub enum VariantScalar {
    Boolean(Boolean),
    SByte(SByte),
    Byte(Byte),
    Int16(Int16),
    UInt16(UInt16),
    Int32(Int32),
    UInt32(UInt32),
    Int64(Int64),
    UInt64(UInt64),
    Float(Float),
    Double(Double),
    String(Box<String>),
    DateTime(DateTime),
    Guid(Box<Guid>),
    ByteString(Box<ByteString>),
    XmlElement(Box<XmlElement>),
    NodeId(Box<NodeId>),
    ExpandedNodeId(Box<ExpandedNodeId>),
    StatusCode(StatusCode),
    QualifiedName(Box<QualifiedName>),
    LocalizedText(Box<LocalizedText>),
    ExtensionObject(Box<ExtensionObject>),
    DataValue(Box<DataValue>),
    Variant(Box<Variant>),
    DiagnosticInfo(Box<DiagnosticInfo>),
}

pub enum VariantArray {
    Boolean(Vec<Boolean>),
    SByte(Vec<SByte>),
    Byte(Vec<Byte>),
    Int16(Vec<Int16>),
    UInt16(Vec<UInt16>),
    Int32(Vec<Int32>),
    UInt32(Vec<UInt32>),
    Int64(Vec<Int64>),
    UInt64(Vec<UInt64>),
    Float(Vec<Float>),
    Double(Vec<Double>),
    String(Vec<String>),
    DateTime(Vec<DateTime>),
    Guid(Vec<Guid>),
    ByteString(Vec<ByteString>),
    XmlElement(Vec<XmlElement>),
    NodeId(Vec<NodeId>),
    ExpandedNodeId(Vec<ExpandedNodeId>),
    StatusCode(Vec<StatusCode>),
    QualifiedName(Vec<QualifiedName>),
    LocalizedText(Vec<LocalizedText>),
    ExtensionObject(Vec<ExtensionObject>),
    DataValue(Vec<DataValue>),
    Variant(Vec<Variant>),
    DiagnosticInfo(Vec<DiagnosticInfo>),
}
