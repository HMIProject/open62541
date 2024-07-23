use open62541_sys::UA_DataType;

use crate::ua;

pub trait Attributes {
    /// Gets associated node class.
    fn node_class(&self) -> ua::NodeClass;

    /// Gets associated attribute type.
    fn attribute_type(&self) -> *const UA_DataType;

    /// Sets display name.
    #[must_use]
    fn with_display_name(self, display_name: &ua::LocalizedText) -> Self;

    /// Gets generic [`ua::NodeAttributes`] type.
    fn as_node_attributes(&self) -> &ua::NodeAttributes;
}
