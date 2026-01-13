use std::pin::Pin;

use open62541_sys::UA_StructureDescription;

use crate::{DataType as _, Result, ua};

crate::data_type!(StructureDescription);

impl StructureDescription {
    pub(crate) fn new(
        data_type_id: ua::NodeId,
        name: ua::QualifiedName,
        definition: ua::StructureDefinition,
    ) -> Self {
        Self(UA_StructureDescription {
            dataTypeId: data_type_id.into_raw(),
            name: name.into_raw(),
            structureDefinition: definition.into_raw(),
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
    pub fn structure_definition(&self) -> &ua::StructureDefinition {
        ua::StructureDefinition::raw_ref(&self.0.structureDefinition)
    }

    pub fn to_data_type(
        &self,
        custom_types: Option<Pin<&ua::DataTypeArray>>,
    ) -> Result<ua::DataType> {
        ua::DataType::from_description(ua::ExtensionObject::new(self), custom_types)
    }
}
