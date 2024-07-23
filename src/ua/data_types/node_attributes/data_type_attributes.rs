use crate::ua;

impl super::DataTypeAttributes {
    #[must_use]
    pub const fn handle_node_class(&self) -> ua::NodeClass {
        ua::NodeClass::DATATYPE
    }
}
