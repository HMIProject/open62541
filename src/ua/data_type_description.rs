use crate::ua;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum DataTypeDescription {
    Structure(ua::StructureDescription),
    Enum(ua::EnumDescription),
}

impl DataTypeDescription {
    #[must_use]
    pub fn data_type_id(&self) -> &ua::NodeId {
        match self {
            Self::Structure(description) => description.data_type_id(),
            Self::Enum(description) => description.data_type_id(),
        }
    }

    #[must_use]
    pub fn name(&self) -> &ua::QualifiedName {
        match self {
            Self::Structure(description) => description.name(),
            Self::Enum(description) => description.name(),
        }
    }

    #[must_use]
    pub fn to_definition(&self) -> ua::DataTypeDefinition {
        match self {
            Self::Structure(description) => {
                ua::DataTypeDefinition::Structure(description.structure_definition().to_owned())
            }
            Self::Enum(description) => {
                ua::DataTypeDefinition::Enum(description.enum_definition().to_owned())
            }
        }
    }
}
