use open62541_sys::UA_EnumDescription;

use crate::{DataType as _, ua};

crate::data_type!(EnumDescription);
crate::member_accessors!(EnumDescription {
    dataTypeId: &ua::NodeId,
    name: &ua::QualifiedName,
    enumDefinition: &ua::EnumDefinition,
    builtInType: u8
});

impl EnumDescription {
    // TODO: Find abstraction for `built_in_type`.
    #[expect(dead_code, reason = "unused for now")]
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
