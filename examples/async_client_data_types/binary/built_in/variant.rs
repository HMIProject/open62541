use bytes::Bytes;

use crate::{
    binary::{BuiltInTypeId, StatelessBinaryReader},
    data_types::{Array, Byte, Int32, Variant, VariantArray, VariantScalar},
};

// [Part 6: 5.2.2.16 Variant](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.16)
impl StatelessBinaryReader for Variant {
    fn read(data: &mut Bytes) -> Self {
        read_variant(data)
    }
}

// [Part 6: 5.2.2.16 Variant](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.16)
fn read_variant(data: &mut Bytes) -> Variant {
    let encoding_mask = Byte::read(data);

    let type_id = encoding_mask.0 & 0b0011_1111;
    let array_dimensions = (encoding_mask.0 & 0b0100_0000) != 0x00;
    let array_of_values = (encoding_mask.0 & 0b1000_0000) != 0x00;

    if type_id == 0 {
        return Variant::Null;
    }
    assert!(1 <= type_id && type_id <= 31);
    let type_id = BuiltInTypeId::from_u8(type_id).unwrap_or(BuiltInTypeId::ByteString);

    if !array_of_values {
        let value = read_variant_scalar(type_id, data);

        assert!(!array_dimensions);
        return Variant::Scalar(value);
    }

    let array_length = Int32::read(data).0;
    let values = if array_length == -1 {
        None
    } else {
        let length = usize::try_from(array_length).unwrap();
        Some(read_variant_array(type_id, length, data))
    };

    let array_dimensions = array_dimensions
        .then(|| {
            let dimensions = Array::<Int32>::read_one_dimensional(data);
            dimensions.iter().map(|dimensions| {
                dimensions
                    .map(|dimension| dimension.0)
                    .collect::<Box<[_]>>()
            })
        })
        .flatten();

    let Some(values) = values else {
        assert!(array_dimensions.is_none());
        return Variant::Array(None);
    };

    let array_dimensions = array_dimensions.unwrap_or_else(|| Box::new([array_length]));
    Variant::Array(Some((values, array_dimensions)))
}

// [Part 6: 5.2.2.16 Variant](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.16)
fn read_variant_scalar(type_id: BuiltInTypeId, data: &mut Bytes) -> VariantScalar {
    macro_rules! read {
        ($( $name:ident ),+ $( , )?) => {
            match type_id {
                $(
                    BuiltInTypeId::$name => {
                        let value = crate::data_types::$name::read(data);

                        VariantScalar::$name(value.into())
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

// [Part 6: 5.2.2.16 Variant](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.16)
fn read_variant_array(type_id: BuiltInTypeId, length: usize, data: &mut Bytes) -> VariantArray {
    macro_rules! read {
        ($( $name:ident ),+ $( , )?) => {
            match type_id {
                $(
                    BuiltInTypeId::$name => {
                        let elements = (0..length).map(|_| {
                            crate::data_types::$name::read(data)
                        }).collect();

                        VariantArray::$name(elements)
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
