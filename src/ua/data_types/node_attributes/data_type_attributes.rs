use crate::ua;

impl super::DataTypeAttributes {
    #[must_use]
    pub const fn handle_node_class(&self) -> ua::NodeClass {
        ua::NodeClass::DATATYPE
    }

    #[must_use]
    pub const fn handle_check_node_type_definition(&self) -> bool {
        false
    }
}
