//! [5.2 OPC UA Binary](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2)

mod array;
mod built_in;
mod decimal;
mod enumeration;
mod index;
mod locale_id;
mod structure;

use std::sync::Arc;

use bytes::Bytes;

use crate::data_types::{DataTypeDefinition, NodeId, StructureDefinition};

pub(crate) trait StatelessBinaryReader {
    #[must_use]
    fn read(data: &mut Bytes) -> Self;
}

pub(crate) trait BinaryReader
where
    Self: Sized,
{
    #[must_use]
    fn read_with_context(context: &BinaryReaderContext, data: &mut Bytes) -> Self;
}

#[derive(Clone)]
pub(crate) struct BinaryReaderContext {
    structure_definition: Option<StructureDefinition>,
    find_data_type_definition: Arc<dyn Fn(&NodeId) -> (&NodeId, &DataTypeDefinition)>,
}

impl BinaryReaderContext {
    pub(crate) fn with_structure_definition(
        self,
        structure_definition: StructureDefinition,
    ) -> Self {
        Self {
            structure_definition: Some(structure_definition),
            ..self
        }
    }
}

#[derive(Debug, Clone, Copy)]
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
    #[must_use]
    fn from_u8(type_id: u8) -> Option<Self> {
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

    #[must_use]
    fn from_node_id(data_type: &NodeId) -> Option<Self> {
        Self::from_u8(u8::try_from(data_type.as_ns0()?).ok()?)
    }
}
