use crate::data_types::{
    Array, Boolean, Byte, ByteString, DataValue, DateTime, DiagnosticInfo, Double, ExpandedNodeId,
    ExtensionObject, Float, Guid, Int16, Int32, Int64, LocalizedText, NodeId, QualifiedName, SByte,
    StatusCode, String, UInt16, UInt32, UInt64, XmlElement,
};

// [Part 6: 5.1.9 Variant](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.9)
// [Part 6: 5.1.11 Null, Empty and Zero-Length Arrays](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.11)
#[derive(Debug, Clone)]
pub enum Variant {
    Null,
    Scalar(VariantScalar),
    Array(Option<(VariantArray, Box<[i32]>)>),
}

impl Variant {
    #[must_use]
    pub fn scalar<T>(value: T) -> Self
    where
        T: Into<VariantScalar>,
    {
        Self::Scalar(value.into())
    }

    pub fn array<T>(value: Array<T>) -> Self
    where
        Vec<T>: Into<VariantArray>,
    {
        match value {
            Array(None) => Self::Array(None),
            Array(Some((elements, array_dimensions))) => {
                let elements = elements.into_vec().into();
                Self::Array(Some((elements, array_dimensions)))
            }
        }
    }

    #[must_use]
    pub fn null() -> Self {
        Self::Null
    }

    #[must_use]
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    #[must_use]
    pub fn is_scalar(&self) -> bool {
        matches!(self, Self::Scalar(_))
    }

    #[must_use]
    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum VariantArray {
    Boolean(Box<[Boolean]>),
    SByte(Box<[SByte]>),
    Byte(Box<[Byte]>),
    Int16(Box<[Int16]>),
    UInt16(Box<[UInt16]>),
    Int32(Box<[Int32]>),
    UInt32(Box<[UInt32]>),
    Int64(Box<[Int64]>),
    UInt64(Box<[UInt64]>),
    Float(Box<[Float]>),
    Double(Box<[Double]>),
    String(Box<[String]>),
    DateTime(Box<[DateTime]>),
    Guid(Box<[Guid]>),
    ByteString(Box<[ByteString]>),
    XmlElement(Box<[XmlElement]>),
    NodeId(Box<[NodeId]>),
    ExpandedNodeId(Box<[ExpandedNodeId]>),
    StatusCode(Box<[StatusCode]>),
    QualifiedName(Box<[QualifiedName]>),
    LocalizedText(Box<[LocalizedText]>),
    ExtensionObject(Box<[ExtensionObject]>),
    DataValue(Box<[DataValue]>),
    Variant(Box<[Variant]>),
    DiagnosticInfo(Box<[DiagnosticInfo]>),
}

macro_rules! impl_into_variant_scalar {
    ( $name:ident(()) ) => {
        impl From<$name> for VariantScalar {
            fn from(value: $name) -> Self {
                Self::$name(value)
            }
        }
    };
    ( $name:ident(Box) ) => {
        impl From<$name> for VariantScalar {
            fn from(value: $name) -> Self {
                Self::$name(Box::new(value))
            }
        }
    };
}

macro_rules! impl_into_variant {
    ( $( $name:ident($type:tt) ),+ $( , )? ) => {
        $(
            impl_into_variant_scalar!($name($type));

            impl From<Vec<$name>> for VariantArray {
                fn from(value: Vec<$name>) -> Self {
                    Self::$name(value.into_boxed_slice())
                }
            }
        )+
    };
}

impl_into_variant!(
    Boolean(()),
    SByte(()),
    Byte(()),
    Int16(()),
    UInt16(()),
    Int32(()),
    UInt32(()),
    Int64(()),
    UInt64(()),
    Float(()),
    Double(()),
    String(Box),
    DateTime(()),
    Guid(Box),
    ByteString(Box),
    XmlElement(Box),
    NodeId(Box),
    ExpandedNodeId(Box),
    StatusCode(()),
    QualifiedName(Box),
    LocalizedText(Box),
    ExtensionObject(Box),
    DataValue(Box),
    Variant(Box),
    DiagnosticInfo(Box),
);
