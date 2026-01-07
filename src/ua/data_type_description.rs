use crate::ua;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum DataTypeDescription {
    Structure(ua::StructureDescription),
    Enum(ua::EnumDescription),
    Unknown(ua::ExtensionObject),
}

impl DataTypeDescription {
    pub(crate) fn into_abstract(self) -> ua::ExtensionObject {
        match self {
            Self::Structure(value) => ua::ExtensionObject::new(&value),
            Self::Enum(value) => ua::ExtensionObject::new(&value),
            Self::Unknown(value) => value,
        }
    }
}
