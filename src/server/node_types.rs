use crate::{ua, Attributes, DataType};

use super::NodeContext;

pub struct Node<T: Attributes> {
    id: ua::NodeId,
    parent_node_id: ua::NodeId,
    reference_type_id: ua::NodeId,
    browse_name: ua::QualifiedName,
    type_definition: ua::NodeId,
    context: Option<NodeContext>,
    attributes: T,
}

impl<T: Attributes + DataType> Node<T> {
    pub fn init() -> Node<T> {
        Node {
            id: ua::NodeId::null(),
            parent_node_id: ua::NodeId::null(),
            reference_type_id: ua::NodeId::null(),
            browse_name: ua::QualifiedName::init(),
            type_definition: ua::NodeId::null(),
            context: None,
            attributes: T::init(),
        }
    }

    pub fn new(
        parent_node_id: ua::NodeId,
        reference_type_id: ua::NodeId,
        browse_name: ua::QualifiedName,
        attributes: T,
    ) -> Node<T> {
        Node {
            id: ua::NodeId::null(),
            parent_node_id,
            reference_type_id,
            browse_name,
            type_definition: ua::NodeId::null(),
            context: None,
            attributes,
        }
    }

    pub fn with_type_definition(mut self, type_definition: ua::NodeId) -> Self {
        self.type_definition = type_definition;
        self
    }

    pub const fn id(&self) -> &ua::NodeId {
        &self.id
    }

    #[must_use]
    pub const fn type_definition(&self) -> &ua::NodeId {
        &self.type_definition
    }

    #[must_use]
    pub(crate) const fn context(&self) -> &Option<NodeContext> {
        &self.context
    }

    #[must_use]
    pub const fn attributes(&self) -> &T {
        &self.attributes
    }

    #[must_use]
    pub const fn parent_node_id(&self) -> &ua::NodeId {
        &self.parent_node_id
    }

    #[must_use]
    pub const fn reference_type_id(&self) -> &ua::NodeId {
        &self.reference_type_id
    }

    #[must_use]
    pub const fn browse_name(&self) -> &ua::QualifiedName {
        &self.browse_name
    }
}

#[derive(Debug, Clone)]
pub struct ObjectNode {
    pub requested_new_node_id: ua::NodeId,
    pub parent_node_id: ua::NodeId,
    pub reference_type_id: ua::NodeId,
    pub browse_name: ua::QualifiedName,
    pub type_definition: ua::NodeId,
    pub attributes: ua::ObjectAttributes,
}

#[derive(Debug, Clone)]
pub struct VariableNode {
    pub requested_new_node_id: ua::NodeId,
    pub parent_node_id: ua::NodeId,
    pub reference_type_id: ua::NodeId,
    pub browse_name: ua::QualifiedName,
    pub type_definition: ua::NodeId,
    pub attributes: ua::VariableAttributes,
}
