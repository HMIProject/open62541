use crate::{
    ua::{self},
    Attributes,
};

use super::NodeContext;

pub struct Node<T: Attributes> {
    pub id: ua::NodeId,
    pub parent_node_id: ua::NodeId,
    pub reference_type_id: ua::NodeId,
    pub browse_name: ua::QualifiedName,
    pub type_definition: Option<ua::NodeId>,
    pub context: Option<NodeContext>,
    pub attributes: T,
}

impl<T: Attributes> Node<T> {
    pub fn get_type_definition(&self) -> ua::NodeId {
        todo!("Not implemented yet!");
        #[allow(unreachable_code)]
        ua::NodeId::null()
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
