use crate::{
    ua::{self},
    Attributes,
};

use super::NodeContext;

pub struct Node<T: Attributes> {
    pub id: Option<ua::NodeId>,
    pub parent_node_id: ua::NodeId,
    pub reference_type_id: ua::NodeId,
    pub browse_name: ua::QualifiedName,
    pub type_definition: Option<ua::NodeId>,
    pub context: Option<NodeContext>,
    pub attributes: T,
}

impl<T: Attributes> Node<T> {
    /// # Panics
    ///
    /// This method panics when the `type_definition` field is None,
    /// and the node attribute type is `Variable`, `VariableType`
    /// or `Object`
    #[must_use]
    pub fn get_type_definition(&self) -> ua::NodeId {
        if self.attributes.check_node_type_definition() {
            self.type_definition
                .as_ref()
                .expect("Type definition must be specified for this node type!")
                .clone()
        } else {
            ua::NodeId::null()
        }
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
