use open62541_sys::{UA_DataType, UA_Server};

use crate::{ua, DataType, Result};

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

pub trait Readable: Sized {
    /// Reads a attribute from a server node
    ///
    /// # Usage
    ///
    /// Make sure to check the `attribute_id` is compatible
    /// to the data type.
    ///
    /// # Errors
    ///
    /// This errors when the `attribute_id` isn't compatible
    /// or the attribute could not be read.
    fn read(
        server: *const UA_Server,
        attribute_id: ua::AttributeId,
        node_id: &ua::NodeId,
    ) -> Result<Self>;
}
