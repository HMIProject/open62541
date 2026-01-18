use bytes::Bytes;

use crate::{
    binary::BinaryReader,
    data_types::{Byte, Int32, Variant, VariantArray, VariantScalar},
};

// [Part 6: 5.2.2.16 Variant](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.16)
impl BinaryReader for Variant {
    fn read(data: &mut Bytes) -> Self {
        let encoding_mask = Byte::read(data);
        let type_id = encoding_mask.0 & 0b0011_1111;
        let array_dimensions = (encoding_mask.0 & 0b0100_0000) != 0x00;
        let array_of_values = (encoding_mask.0 & 0b1000_0000) != 0x00;
        if type_id == 0 {
            return Self::Null;
        }
        assert!(1 <= type_id && type_id <= 31);
        let type_id = BuiltInTypeId::new(type_id).unwrap_or(BuiltInTypeId::ByteString);
        if array_of_values {
            let array_length = Int32::read(data);
            let values = if array_length.0 == -1 {
                None
            } else {
                let array_length = usize::try_from(array_length.0).unwrap();
                Some(read_variant_array(type_id, array_length, data))
            };
            let array_dimensions =
                if array_dimensions && let Some(array_dimensions) = read_array::<Int32>(data) {
                    let array_dimensions = array_dimensions
                        .into_iter()
                        .map(|array_dimension| array_dimension.0)
                        .collect::<Vec<_>>();
                    Some(array_dimensions)
                } else {
                    None
                };
            Self::Array(values, array_dimensions)
        } else {
            let value = read_variant_scalar(type_id, data);
            assert!(!array_dimensions);
            Self::Scalar(value)
        }
    }
}

#[derive(Clone, Copy)]
enum BuiltInTypeId {
    Boolean,
    SByte,
    Byte,
    Int16,
    UInt16,
    Int32,
    UInt32,
    Int64,
    UInt64,
    Float,
    Double,
    String,
    DateTime,
    Guid,
    ByteString,
    XmlElement,
    NodeId,
    ExpandedNodeId,
    StatusCode,
    QualifiedName,
    LocalizedText,
    ExtensionObject,
    DataValue,
    Variant,
    DiagnosticInfo,
}

impl BuiltInTypeId {
    fn new(type_id: u8) -> Option<Self> {
        Some(match type_id {
            1 => Self::Boolean,
            2 => Self::SByte,
            3 => Self::Byte,
            4 => Self::Int16,
            5 => Self::UInt16,
            6 => Self::Int32,
            7 => Self::UInt32,
            8 => Self::Int64,
            9 => Self::UInt64,
            10 => Self::Float,
            11 => Self::Double,
            12 => Self::String,
            13 => Self::DateTime,
            14 => Self::Guid,
            15 => Self::ByteString,
            16 => Self::XmlElement,
            17 => Self::NodeId,
            18 => Self::ExpandedNodeId,
            19 => Self::StatusCode,
            20 => Self::QualifiedName,
            21 => Self::LocalizedText,
            22 => Self::ExtensionObject,
            23 => Self::DataValue,
            24 => Self::Variant,
            25 => Self::DiagnosticInfo,
            _ => return None,
        })
    }
}

fn read_variant_scalar(type_id: BuiltInTypeId, data: &mut Bytes) -> VariantScalar {
    macro_rules! read {
        ($( $name:ident ),+ $( , )?) => {
            match type_id {
                $(
                    BuiltInTypeId::$name => VariantScalar::$name(
                        crate::data_types::$name::read(data).into(),
                    ),
                )+
            }
        };
    }

    read!(
        Boolean,
        SByte,
        Byte,
        Int16,
        UInt16,
        Int32,
        UInt32,
        Int64,
        UInt64,
        Float,
        Double,
        String,
        DateTime,
        Guid,
        ByteString,
        XmlElement,
        NodeId,
        ExpandedNodeId,
        StatusCode,
        QualifiedName,
        LocalizedText,
        ExtensionObject,
        DataValue,
        Variant,
        DiagnosticInfo,
    )
}

fn read_variant_array(type_id: BuiltInTypeId, length: usize, data: &mut Bytes) -> VariantArray {
    macro_rules! read {
        ($( $name:ident ),+ $( , )?) => {
            match type_id {
                $(
                    BuiltInTypeId::$name => {
                        VariantArray::$name((0..length).map(|_| {
                            crate::data_types::$name::read(data)
                        }).collect())
                    },
                )+
            }
        };
    }

    read!(
        Boolean,
        SByte,
        Byte,
        Int16,
        UInt16,
        Int32,
        UInt32,
        Int64,
        UInt64,
        Float,
        Double,
        String,
        DateTime,
        Guid,
        ByteString,
        XmlElement,
        NodeId,
        ExpandedNodeId,
        StatusCode,
        QualifiedName,
        LocalizedText,
        ExtensionObject,
        DataValue,
        Variant,
        DiagnosticInfo,
    )
}

fn read_array<T>(data: &mut Bytes) -> Option<Vec<T>>
where
    T: BinaryReader,
{
    let length = Int32::read(data);
    if length.0 == -1 {
        return None;
    }
    let length = usize::try_from(length.0).unwrap();
    Some((0..length).map(|_| T::read(data)).collect())
}
