use open62541_sys::{UA_NS0ID_ENUMDEFINITION, UA_NS0ID_STRUCTUREDEFINITION};

use crate::{DataTypeExt, Error, Result, ua};

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum DataTypeDefinition {
    Structure(ua::StructureDefinition),
    Enum(ua::EnumDefinition),
}

impl DataTypeDefinition {
    pub(crate) fn from_abstract(value: ua::Variant) -> Result<Self> {
        let Some(type_id) = value.type_id() else {
            return Err(Error::Internal("require type ID for data type definition"));
        };

        Ok(
            if *type_id == ua::NodeId::ns0(UA_NS0ID_STRUCTUREDEFINITION) {
                // PANIC: We have checked that the expected type ID is set.
                Self::Structure(value.into_scalar().expect("structure definition"))
            } else if *type_id == ua::NodeId::ns0(UA_NS0ID_ENUMDEFINITION) {
                // PANIC: We have checked that the expected type ID is set.
                Self::Enum(value.into_scalar().expect("enum definition"))
            } else {
                return Err(Error::Internal("unsupported data type definition"));
            },
        )
    }

    #[must_use]
    pub(crate) fn into_abstract(self) -> ua::Variant {
        match self {
            Self::Structure(value) => ua::Variant::scalar(value),
            Self::Enum(value) => ua::Variant::scalar(value),
        }
    }
}

impl DataTypeExt for DataTypeDefinition {
    type Inner = ua::Variant;

    fn from_inner(value: Self::Inner) -> Result<Self> {
        Self::from_abstract(value)
    }

    fn into_inner(self) -> Self::Inner {
        self.into_abstract()
    }
}
