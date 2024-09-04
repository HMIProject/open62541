use std::fmt;

use crate::{ua, Attributes, DataType};

use crate::server::NodeContext;

pub struct Node<T> {
    pub(crate) requested_new_node_id: Option<ua::NodeId>,
    pub(crate) parent_node_id: ua::NodeId,
    pub(crate) reference_type_id: ua::NodeId,
    pub(crate) browse_name: ua::QualifiedName,
    pub(crate) type_definition: ua::NodeId,
    pub(crate) attributes: T,
    pub(crate) context: Option<NodeContext>,
}

impl<T: Attributes> Node<T> {
    #[must_use]
    pub fn init() -> Self {
        Self {
            requested_new_node_id: None,
            parent_node_id: ua::NodeId::null(),
            reference_type_id: ua::NodeId::null(),
            browse_name: ua::QualifiedName::init(),
            type_definition: ua::NodeId::null(),
            attributes: T::init(),
            context: None,
        }
    }

    #[must_use]
    pub fn new(
        parent_node_id: ua::NodeId,
        reference_type_id: ua::NodeId,
        browse_name: ua::QualifiedName,
        attributes: T,
    ) -> Self {
        Self {
            requested_new_node_id: None,
            parent_node_id,
            reference_type_id,
            browse_name,
            type_definition: ua::NodeId::null(),
            attributes,
            context: None,
        }
    }

    #[must_use]
    pub fn with_requested_new_node_id(mut self, requested_new_node_id: ua::NodeId) -> Self {
        self.requested_new_node_id = Some(requested_new_node_id);
        self
    }

    #[must_use]
    pub fn with_type_definition(mut self, type_definition: ua::NodeId) -> Self {
        self.type_definition = type_definition;
        self
    }

    #[must_use]
    pub const fn requested_new_node_id(&self) -> Option<&ua::NodeId> {
        self.requested_new_node_id.as_ref()
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

    #[must_use]
    pub const fn type_definition(&self) -> &ua::NodeId {
        &self.type_definition
    }

    #[must_use]
    pub const fn attributes(&self) -> &T {
        &self.attributes
    }
}

impl<T: Attributes> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            requested_new_node_id,
            parent_node_id,
            reference_type_id,
            browse_name,
            type_definition,
            attributes,
            context: _,
        } = self;

        f.debug_struct("Node")
            .field("requested_new_node_id", requested_new_node_id)
            .field("parent_node_id", parent_node_id)
            .field("reference_type_id", reference_type_id)
            .field("browse_name", browse_name)
            .field("type_definition", type_definition)
            .field("attributes", attributes)
            .finish_non_exhaustive()
    }
}

#[derive(Debug, Clone)]
pub struct ObjectNode {
    pub requested_new_node_id: Option<ua::NodeId>,
    pub parent_node_id: ua::NodeId,
    pub reference_type_id: ua::NodeId,
    pub browse_name: ua::QualifiedName,
    pub type_definition: ua::NodeId,
    pub attributes: ua::ObjectAttributes,
}

#[derive(Debug, Clone)]
pub struct VariableNode {
    pub requested_new_node_id: Option<ua::NodeId>,
    pub parent_node_id: ua::NodeId,
    pub reference_type_id: ua::NodeId,
    pub browse_name: ua::QualifiedName,
    pub type_definition: ua::NodeId,
    pub attributes: ua::VariableAttributes,
}

#[derive(Debug, Clone)]
pub struct MethodNode {
    pub requested_new_node_id: Option<ua::NodeId>,
    pub parent_node_id: ua::NodeId,
    pub reference_type_id: ua::NodeId,
    pub browse_name: ua::QualifiedName,
    pub attributes: ua::MethodAttributes,
    pub input_arguments: ua::Array<ua::Argument>,
    pub input_arguments_requested_new_node_id: Option<ua::NodeId>,
    pub output_arguments: ua::Array<ua::Argument>,
    pub output_arguments_requested_new_node_id: Option<ua::NodeId>,
}
