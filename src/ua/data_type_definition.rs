use open62541_sys::{UA_NS0ID_ENUMDEFINITION, UA_NS0ID_STRUCTUREDEFINITION};

use crate::{ua, DataTypeExt};

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum DataTypeDefinition {
    Structure(ua::StructureDefinition),
    Enum(ua::EnumDefinition),
    Unknown(ua::Variant),
}

impl DataTypeDefinition {
    pub(crate) fn new(value: ua::Variant) -> Self {
        let Some(type_id) = value.type_id() else {
            return Self::Unknown(value);
        };

        if *type_id == ua::NodeId::ns0(UA_NS0ID_STRUCTUREDEFINITION) {
            // PANIC: We have checked that the expected type ID is set.
            Self::Structure(value.into_scalar().expect("require structure definition"))
        } else if *type_id == ua::NodeId::ns0(UA_NS0ID_ENUMDEFINITION) {
            // PANIC: We have checked that the expected type ID is set.
            Self::Enum(value.into_scalar().expect("require enum definition"))
        } else {
            Self::Unknown(value)
        }
    }

    pub(crate) fn into_abstract(self) -> ua::Variant {
        match self {
            Self::Structure(value) => ua::Variant::scalar(value),
            Self::Enum(value) => ua::Variant::scalar(value),
            Self::Unknown(value) => value,
        }
    }
}

impl DataTypeExt for DataTypeDefinition {
    type Inner = ua::Variant;

    fn from_inner(value: Self::Inner) -> Self {
        Self::new(value)
    }

    fn into_inner(self) -> Self::Inner {
        self.into_abstract()
    }
}
