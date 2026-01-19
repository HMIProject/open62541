use crate::data_types::{NodeId, StructureField, StructureType};

// [Part 3: 8.48 StructureDefinition](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.48)
// [Part 5: 12.2.12.5 StructureDefinition](https://reference.opcfoundation.org/Core/Part5/v105/docs/12.2.12.5)
#[derive(Debug, Clone)]
pub struct StructureDefinition {
    pub default_encoding_id: NodeId,
    pub base_data_type: NodeId,
    pub structure_type: StructureType,
    pub fields: Box<[StructureField]>,
}
