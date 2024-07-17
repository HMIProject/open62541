use crate::ua;

impl super::ObjectAttributes {
    #[must_use]
    pub const fn handle_node_class(&self) -> ua::NodeClass {
        ua::NodeClass::OBJECT
    }

    #[must_use]
    pub const fn handle_check_node_type_definition(&self) -> bool {
        true
    }
}
