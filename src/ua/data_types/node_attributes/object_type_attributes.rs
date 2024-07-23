use crate::ua;

impl super::ObjectTypeAttributes {
    #[must_use]
    pub const fn handle_node_class(&self) -> ua::NodeClass {
        ua::NodeClass::OBJECTTYPE
    }
}
