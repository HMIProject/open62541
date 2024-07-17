use crate::{ua, DataType as _};

impl super::VariableAttributes {
    #[must_use]
    pub fn with_data_type(mut self, data_type: &ua::NodeId) -> Self {
        data_type.clone_into_raw(&mut self.0.dataType);
        self
    }

    #[must_use]
    pub fn with_access_level(mut self, access_level: &ua::AccessLevel) -> Self {
        self.0.accessLevel = access_level.as_u8();
        self
    }

    #[must_use]
    pub fn with_value_rank(mut self, rank: i32) -> Self {
        self.0.valueRank = rank;
        self
    }

    #[must_use]
    pub const fn handle_node_class(&self) -> ua::NodeClass {
        ua::NodeClass::VARIABLE
    }

    #[must_use]
    pub const fn handle_check_node_type_definition(&self) -> bool {
        true
    }
}
