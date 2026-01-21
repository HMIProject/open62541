use crate::{DataType, ua};

crate::data_type!(StructureDefinition);

impl StructureDefinition {
    #[must_use]
    pub fn default_encoding_id(&self) -> &ua::NodeId {
        ua::NodeId::raw_ref(&self.0.defaultEncodingId)
    }

    #[must_use]
    pub fn base_data_type(&self) -> &ua::NodeId {
        ua::NodeId::raw_ref(&self.0.baseDataType)
    }

    #[must_use]
    pub fn structure_type(&self) -> &ua::StructureType {
        ua::StructureType::raw_ref(&self.0.structureType)
    }

    #[must_use]
    pub fn fields(&self) -> Option<ua::Array<ua::StructureField>> {
        // TODO: Adjust signature to return non-owned value instead.
        ua::Array::from_raw_parts(self.0.fieldsSize, self.0.fields)
    }

    #[must_use]
    pub fn into_description(
        self,
        data_type_id: ua::NodeId,
        name: ua::QualifiedName,
    ) -> ua::StructureDescription {
        ua::StructureDescription::new(data_type_id, name, self)
    }
}
