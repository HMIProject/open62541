use std::pin::Pin;

use open62541_sys::UA_EnumDescription;

use crate::{DataType as _, Result, ua};

crate::data_type!(EnumDescription);

impl EnumDescription {
    // TODO: Find abstraction for `built_in_type`.
    pub(crate) fn new(
        data_type_id: ua::NodeId,
        name: ua::QualifiedName,
        definition: ua::EnumDefinition,
        built_in_type: u8,
    ) -> Self {
        Self(UA_EnumDescription {
            dataTypeId: data_type_id.into_raw(),
            name: name.into_raw(),
            enumDefinition: definition.into_raw(),
            builtInType: built_in_type,
        })
    }

    #[must_use]
    pub fn data_type_id(&self) -> &ua::NodeId {
        ua::NodeId::raw_ref(&self.0.dataTypeId)
    }

    #[must_use]
    pub fn name(&self) -> &ua::QualifiedName {
        ua::QualifiedName::raw_ref(&self.0.name)
    }

    #[must_use]
    pub fn enum_definition(&self) -> &ua::EnumDefinition {
        ua::EnumDefinition::raw_ref(&self.0.enumDefinition)
    }

    // TODO: Encapsulate in better return type.
    #[must_use]
    pub(crate) fn built_in_type(&self) -> u8 {
        self.0.builtInType
    }

    pub fn to_data_type(
        &self,
        custom_types: Option<Pin<&ua::DataTypeArray>>,
    ) -> Result<ua::DataType> {
        ua::DataType::from_description(ua::ExtensionObject::new(self), custom_types)
    }
}
