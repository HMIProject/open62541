use crate::data_types::StructureDefinition;

// [Part 3: 8.47 DataTypeDefinition](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.47)
// [Part 5: 12.2.12.3 DataTypeDefinition](https://reference.opcfoundation.org/Core/Part5/v105/docs/12.2.12.3)
#[derive(Debug, Clone)]
pub enum DataTypeDefinition {
    Structure(StructureDefinition),
}
