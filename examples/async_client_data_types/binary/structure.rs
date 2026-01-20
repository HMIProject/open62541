use std::num::NonZeroU32;

use bytes::Bytes;

use crate::{
    binary::{BinaryReader, BinaryReaderWithContext, BuiltInTypeId},
    data_types::{
        Array, DataTypeDefinition, ExtensionObject, NodeId, Structure, StructureDefinition,
        StructureField, StructureType, UInt32, Variant,
    },
};

// [Part 6: 5.1.7 Structures and Unions](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.7)
// [Part 6: 5.2.6 Structures](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.6)
// [Part 6: 5.2.7 Structures with optional fields](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.7)
// [Part 6: 5.2.8 Unions](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.8)
impl Structure {
    #[must_use]
    pub(crate) fn read(
        structure_definition: &StructureDefinition,
        context: &ReadStructureContext,
        data: &mut Bytes,
    ) -> Self {
        Self::read_with_context((structure_definition, context), data)
    }
}

impl BinaryReaderWithContext<(&StructureDefinition, &ReadStructureContext)> for Structure {
    fn read_with_context(
        (structure_definition, context): (&StructureDefinition, &ReadStructureContext),
        data: &mut Bytes,
    ) -> Self {
        let StructureDefinition {
            structure_type,
            fields,
            ..
        } = structure_definition;

        match structure_type {
            StructureType::Structure => read_structure(context, fields, data),
            StructureType::StructureWithOptionalFields => {
                read_structure_with_optional_fields(context, fields, data)
            }
            StructureType::StructureWithSubtypedValues => {
                read_union_with_subtyped_fields(context, fields, data)
            }
            StructureType::Union => read_union(context, fields, data),
            StructureType::UnionWithSubtypedValues => {
                read_union_with_subtyped_fields(context, fields, data)
            }
        }
    }
}

struct ReadStructureContext {
    find_data_type_definition: Box<dyn Fn(&NodeId) -> (&NodeId, &DataTypeDefinition)>,
}

// [Part 6: 5.2.6 Structures](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.6)
fn read_structure(
    context: &ReadStructureContext,
    fields: &[StructureField],
    data: &mut Bytes,
) -> Structure {
    let field_values = fields
        .iter()
        .map(|field| read_field(context, field, data))
        .collect();

    Structure::Structure(field_values)
}

// [Part 6: 5.2.7 Structures with optional fields](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.7)
fn read_structure_with_optional_fields(
    context: &ReadStructureContext,
    fields: &[StructureField],
    data: &mut Bytes,
) -> Structure {
    let encoding_mask = UInt32::read(data);

    let field_values = fields
        .iter()
        .enumerate()
        .map(|(index, field)| {
            let is_present = encoding_mask.0 & (1 << index) != 0;
            is_present.then(|| read_field(context, field, data))
        })
        .collect();

    Structure::StructureWithOptionalFields(field_values)
}

fn read_structure_with_subtyped_fields(
    context: &ReadStructureContext,
    fields: &[StructureField],
    data: &mut Bytes,
) -> Structure {
    todo!()
}

// [Part 6: 5.2.8 Unions](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.8)
fn read_union(
    context: &ReadStructureContext,
    fields: &[StructureField],
    data: &mut Bytes,
) -> Structure {
    let switch_field = UInt32::read(data);
    if switch_field.0 == 0 {
        return Structure::Union(None);
    }
    let switch_field = NonZeroU32::new(switch_field.0).unwrap();

    let field_index = usize::try_from(switch_field.get())
        .unwrap()
        .checked_sub(1)
        .unwrap();
    let field = fields.get(field_index).unwrap();
    let field_value = read_field(context, field, data);

    Structure::Union(Some((switch_field, field_value)))
}

fn read_union_with_subtyped_fields(
    context: &ReadStructureContext,
    fields: &[StructureField],
    data: &mut Bytes,
) -> Structure {
    todo!()
}

// [Part 6: 5.2.5 Arrays](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.5)
fn read_field(context: &ReadStructureContext, field: &StructureField, data: &mut Bytes) -> Variant {
    if field.is_scalar() {
        read_scalar_field(context, field, data)
    } else if field.is_array() {
        read_array_field(context, field, data)
    } else {
        panic!();
    }
}

fn read_scalar_field(
    context: &ReadStructureContext,
    field: &StructureField,
    data: &mut Bytes,
) -> Variant {
    let StructureField { data_type, .. } = field;

    let Some(type_id) = BuiltInTypeId::from_node_id(data_type) else {
        let (type_id, data_type_definition) = (context.find_data_type_definition)(data_type);

        let value = match data_type_definition {
            DataTypeDefinition::Structure(structure_definition) => {
                let value = Structure::read(structure_definition, context, data);
                ExtensionObject::Structure(type_id.to_owned(), value)
            }
        };

        return Variant::scalar(value);
    };

    macro_rules! read_built_in_scalar {
        ($( $name:ident ),+ $( , )?) => {
            match type_id {
                $(
                    BuiltInTypeId::$name => {
                        let value = crate::data_types::$name::read(data);

                        Variant::scalar(value)
                    },
                )+
            }
        };
    }

    read_built_in_scalar!(
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

fn read_array_field(
    context: &ReadStructureContext,
    field: &StructureField,
    data: &mut Bytes,
) -> Variant {
    let StructureField { data_type, .. } = field;

    let Some(type_id) = BuiltInTypeId::from_node_id(data_type) else {
        let (type_id, data_type_definition) = (context.find_data_type_definition)(data_type);

        let array = match data_type_definition {
            DataTypeDefinition::Structure(structure_definition) => {
                let array = if field.is_one_dimensional_array() {
                    Array::<Structure>::read_one_dimensional_with_context(
                        (structure_definition, context),
                        data,
                    )
                } else if field.is_multi_dimensional_array() {
                    Array::<Structure>::read_multi_dimensional_with_context(
                        (structure_definition, context),
                        data,
                    )
                } else {
                    panic!();
                };

                array.map(|element| ExtensionObject::Structure(type_id.to_owned(), element))
            }
        };

        return Variant::array(array);
    };

    macro_rules! read_built_in_array {
        ($( $name:ident ),+ $( , )?) => {
            match type_id {
                $(
                    BuiltInTypeId::$name => {
                        let array = if field.is_one_dimensional_array() {
                            Array::<crate::data_types::$name>::read_one_dimensional(data)
                        } else if field.is_multi_dimensional_array() {
                            Array::<crate::data_types::$name>::read_multi_dimensional(data)
                        } else {
                            panic!();
                        };

                        Variant::array(array)
                    },
                )+
            }
        };
    }

    read_built_in_array!(
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
