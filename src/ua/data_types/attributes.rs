use open62541_sys::UA_NodeAttributes;

use crate::{
    ua::{
        self, DataTypeAttributes, ObjectAttributes, ObjectTypeAttributes, ReferenceTypeAttributes,
        VariableAttributes, VariableTypeAttributes, ViewAttributes,
    },
    DataType,
};

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
    pub fn display_name(&mut self, locale: &str, name: &str) -> &mut Self {
        let localized_text =
            ua::LocalizedText::new(locale, name).expect("Localized text could not be created!");
        match self {
            Attributes::DataType(ref mut inner) => unsafe {
                // inner.as_ref().displayName = localized_text;
                localized_text.clone_into_raw(&mut inner.as_mut().displayName);
            },
            Attributes::Object(ref mut inner) => unsafe {
                localized_text.clone_into_raw(&mut inner.as_mut().displayName);
            },
            Attributes::ObjectType(ref mut inner) => unsafe {
                localized_text.clone_into_raw(&mut inner.as_mut().displayName);
            },
            Attributes::ReferenceType(ref mut inner) => unsafe {
                localized_text.clone_into_raw(&mut inner.as_mut().displayName);
            },
            Attributes::Variable(ref mut inner) => unsafe {
                localized_text.clone_into_raw(&mut inner.as_mut().displayName);
            },
            Attributes::VariableType(ref mut inner) => unsafe {
                localized_text.clone_into_raw(&mut inner.as_mut().displayName);
            },
            Attributes::View(ref mut inner) => unsafe {
                localized_text.clone_into_raw(&mut inner.as_mut().displayName);
            },
        }
        self
    }

    pub fn value_rank(&mut self, rank: i32) -> &mut Self {
        match self {
            Attributes::DataType(_)
            | Attributes::Object(_)
            | Attributes::ObjectType(_)
            | Attributes::ReferenceType(_)
            | Attributes::View(_) => {
                panic!("No value_rank field available for this Attribute type!");
            }
            Attributes::Variable(ref mut inner) => unsafe {
                inner.as_mut().valueRank = rank;
            },
            Attributes::VariableType(ref mut inner) => unsafe {
                inner.as_mut().valueRank = rank;
            },
        }
        self
    }

    fn generic_node_attributes<T: Clone + crate::data_type::DataType>(
        inner: &T,
    ) -> *const ua::NodeAttributes {
        let node_attributes = unsafe { (*inner).as_ptr().cast::<UA_NodeAttributes>() };
        let node_attributes = unsafe { node_attributes.as_ref().unwrap_unchecked() };
        ua::NodeAttributes::raw_ref(node_attributes)
    }

    #[must_use]
    pub fn as_node_attributes(&self) -> *const ua::NodeAttributes {
        // SAFETY: This transmutes from `Self` to the inner type, and then to `UA_NodeAttributes`, a
        // subset of `UA_DataTypeAttributes` with the same memory layout.
        match self {
            Attributes::DataType(inner) => Self::generic_node_attributes(inner),
            Attributes::Object(inner) => Self::generic_node_attributes(inner),
            Attributes::ObjectType(inner) => Self::generic_node_attributes(inner),
            Attributes::ReferenceType(inner) => Self::generic_node_attributes(inner),
            Attributes::Variable(_) => self.as_variable_attributes().as_node_attributes(),
            Attributes::VariableType(inner) => Self::generic_node_attributes(inner),
            Attributes::View(inner) => Self::generic_node_attributes(inner),
        }
    }

    /// # Panics
    ///
    /// This method will panic if self isn't `VariableAttributes` but another attributes type.
    #[must_use]
    pub fn as_variable_attributes(&self) -> &ua::VariableAttributes {
        match self {
            Attributes::Variable(inner) => inner,
            _ => panic!("Cannot convert this Attribute to VariableAttribute!"),
        }
    }

    #[must_use]
    pub fn data_type(&self) -> *const open62541_sys::UA_DataType {
        match self {
            Attributes::DataType(_) => ua::DataTypeAttributes::data_type(),
            Attributes::Object(_) => ua::ObjectAttributes::data_type(),
            Attributes::ObjectType(_) => ua::ObjectTypeAttributes::data_type(),
            Attributes::ReferenceType(_) => ua::ReferenceTypeAttributes::data_type(),
            Attributes::Variable(_) => ua::VariableAttributes::data_type(),
            Attributes::VariableType(_) => ua::VariableTypeAttributes::data_type(),
            Attributes::View(_) => ua::ViewAttributes::data_type(),
        }
    }

    #[must_use]
    pub const fn node_class(&self) -> ua::NodeClass {
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
