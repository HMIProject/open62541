use std::fmt;

use open62541_sys::UA_DataType;

use crate::{ua, DataType};

/// Node attribute.
///
/// This is used to match the appropriate result types at compile time when reading attributes from
/// nodes. See the following methods for details:
///
/// - [`AsyncClient::read_attribute()`](crate::AsyncClient::read_attribute)
/// - [`Server::read_attribute()`](crate::Server::read_attribute)
pub trait Attribute: fmt::Debug + Copy {
    /// Attribute data type.
    type Value: DataType;

    /// Gets attribute ID.
    fn id(&self) -> ua::AttributeId;
}

/// Server node attributes.
///
/// This is used to allow handling different node types when adding nodes to the server's data tree
/// in [`Server::add_node()`](crate::Server::add_node).
pub trait Attributes: DataType {
    /// Gets associated node class.
    fn node_class(&self) -> ua::NodeClass;

    /// Gets associated attribute type.
    ///
    /// This is [`<Self as DataType>::data_type()`] with a more appropriate name.
    ///
    /// [`<Self as DataType>::data_type()`]: DataType::data_type()
    fn attribute_type(&self) -> *const UA_DataType;

    /// Sets display name.
    #[must_use]
    fn with_display_name(self, display_name: &ua::LocalizedText) -> Self;

    /// Gets generic [`ua::NodeAttributes`] type.
    fn as_node_attributes(&self) -> &ua::NodeAttributes;
}
