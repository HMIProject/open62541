use crate::ua;

impl super::MethodAttributes {
    #[must_use]
    pub const fn handle_node_class(&self) -> ua::NodeClass {
        ua::NodeClass::METHOD
    }

    #[must_use]
    pub const fn handle_check_node_type_definition(&self) -> bool {
        false
    }
}
