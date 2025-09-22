use crate::{DataType, ua};

crate::data_type!(StructureDefinition);

impl StructureDefinition {
    #[must_use]
    pub fn base_data_type(&self) -> &ua::NodeId {
        ua::NodeId::raw_ref(&self.0.baseDataType)
    }
}
