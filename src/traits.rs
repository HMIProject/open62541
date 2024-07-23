use open62541_sys::UA_DataType;

use crate::{ua, DataType};

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
