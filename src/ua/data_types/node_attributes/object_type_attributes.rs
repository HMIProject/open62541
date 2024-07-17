use crate::ua;

impl super::ObjectTypeAttributes {
    #[must_use]
    pub const fn handle_node_class(&self) -> ua::NodeClass {
        ua::NodeClass::OBJECTTYPE
    }

    #[must_use]
    pub const fn handle_check_node_type_definition(&self) -> bool {
        false
    }
}
