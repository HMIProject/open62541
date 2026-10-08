use open62541_sys::UA_StructureType;

use crate::ua;

crate::data_type!(StructureDefinition);
crate::member_accessors!(StructureDefinition {
    defaultEncodingId: &ua::NodeId,
    baseDataType: &ua::NodeId,
    #[enum(UA_StructureType)]
    structureType: ua::StructureType,
    fields: [ua::StructureField]
});

impl StructureDefinition {
    #[must_use]
    pub fn into_description(
        self,
        data_type_id: ua::NodeId,
        name: ua::QualifiedName,
    ) -> ua::StructureDescription {
        ua::StructureDescription::new(data_type_id, name, self)
    }
}
