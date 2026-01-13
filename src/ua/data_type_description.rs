use std::pin::Pin;

use crate::{Result, ua};

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

    pub fn to_data_type(
        &self,
        custom_types: Option<Pin<&ua::DataTypeArray>>,
    ) -> Result<ua::DataType> {
        match self {
            Self::Structure(value) => value.to_data_type(custom_types),
            Self::Enum(value) => value.to_data_type(custom_types),
        }
    }
}
