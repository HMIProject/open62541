use open62541_sys::UA_NodeAttributes;

use crate::{ua::{self, DataTypeAttributes, ObjectAttributes, ObjectTypeAttributes, ReferenceTypeAttributes, VariableAttributes, VariableTypeAttributes, ViewAttributes}, DataType};

#[derive(Debug, Clone)]
pub enum Attributes {
    DataType(DataTypeAttributes),
    Object(ObjectAttributes),
    ObjectType(ObjectTypeAttributes),
    ReferenceType(ReferenceTypeAttributes),
    Variable(VariableAttributes),
    VariableType(VariableTypeAttributes),
    View(ViewAttributes),
}

impl Attributes {

    fn generic_node_attributes<T: Clone + crate::data_type::DataType>(&self, inner: &T) -> &ua::NodeAttributes {
        let node_attributes = unsafe {inner.clone().as_ptr().cast::<UA_NodeAttributes>()};
        let node_attributes = unsafe { node_attributes.as_ref().unwrap_unchecked() };
        ua::NodeAttributes::raw_ref(node_attributes)
    }

    pub fn as_node_attributes(&self) -> &ua::NodeAttributes {
        // SAFETY: This transmutes from `Self` to the inner type, and then to `UA_NodeAttributes`, a
        // subset of `UA_DataTypeAttributes` with the same memory layout.
        match self {
            Attributes::DataType(inner) => self.generic_node_attributes(inner),
            Attributes::Object(inner) => self.generic_node_attributes(inner),
            Attributes::ObjectType(inner) => self.generic_node_attributes(inner),
            Attributes::ReferenceType(inner) => self.generic_node_attributes(inner),
            Attributes::Variable(_) => self.generic_node_attributes(self.as_variable_attributes().as_node_attributes()),
            Attributes::VariableType(inner) => self.generic_node_attributes(inner),
            Attributes::View(inner) => self.generic_node_attributes(inner),
        }
    }

    pub fn as_variable_attributes(&self) -> ua::VariableAttributes {
        match self {
            Attributes::Variable(inner) => inner.clone(),
            _ => panic!("Cannot convert this Attribute to VariableAttribute!")
        }
    }

    pub fn data_type(&self) -> *const open62541_sys::UA_DataType {
        match self {
            Attributes::DataType(_) => ua::VariableAttributes::data_type(),
            Attributes::Object(_) => ua::ObjectAttributes::data_type(),
            Attributes::ObjectType(_) => ua::ObjectTypeAttributes::data_type(),
            Attributes::ReferenceType(_) => ua::ReferenceTypeAttributes::data_type(),
            Attributes::Variable(_) => ua::VariableAttributes::data_type(),
            Attributes::VariableType(_) => ua::VariableTypeAttributes::data_type(),
            Attributes::View(_) => ua::ViewAttributes::data_type(),
        }
    }

    pub fn node_class(&self) -> ua::NodeClass {
        match self {
            Attributes::DataType(_) => ua::NodeClass::DATATYPE,
            Attributes::Object(_) => ua::NodeClass::OBJECT,
            Attributes::ObjectType(_) => ua::NodeClass::OBJECTTYPE,
            Attributes::ReferenceType(_) => ua::NodeClass::REFERENCETYPE,
            Attributes::Variable(_) => ua::NodeClass::VARIABLE,
            Attributes::VariableType(_) => ua::NodeClass::VARIABLETYPE,
            Attributes::View(_) => ua::NodeClass::VIEW,
        }
    }
}
