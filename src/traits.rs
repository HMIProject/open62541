use crate::ua;

pub trait Attributes {
    fn as_node_attributes(&self) -> &ua::NodeAttributes;
    #[must_use]
    fn with_display_name(self, locale: &str, name: &str) -> Self;
    fn node_class(&self) -> ua::NodeClass;
}
