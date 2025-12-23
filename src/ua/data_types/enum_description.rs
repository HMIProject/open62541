use open62541_sys::UA_EnumDescription;

use crate::{DataType as _, ua};

crate::data_type!(EnumDescription);

impl EnumDescription {
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
}
