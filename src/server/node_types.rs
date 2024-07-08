use crate::ua;

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
pub struct ObjectTypeNode {
    pub requested_new_node_id: ua::NodeId,
    pub parent_node_id: ua::NodeId,
    pub reference_type_id: ua::NodeId,
    pub browse_name: ua::QualifiedName,
    pub attributes: ua::ObjectTypeAttributes,
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

#[derive(Debug, Clone)]
pub struct VariableTypeNode {
    pub requested_new_node_id: ua::NodeId,
    pub parent_node_id: ua::NodeId,
    pub reference_type_id: ua::NodeId,
    pub browse_name: ua::QualifiedName,
    pub type_definition: ua::NodeId,
    pub attributes: ua::VariableTypeAttributes,
}

#[derive(Debug, Clone)]
pub struct ViewNode {
    pub requested_new_node_id: ua::NodeId,
    pub parent_node_id: ua::NodeId,
    pub reference_type_id: ua::NodeId,
    pub browse_name: ua::QualifiedName,
    pub attributes: ua::ViewAttributes,
}

#[derive(Debug, Clone)]
pub struct ReferenceTypeNode {
    pub requested_new_node_id: ua::NodeId,
    pub parent_node_id: ua::NodeId,
    pub reference_type_id: ua::NodeId,
    pub browse_name: ua::QualifiedName,
    pub attributes: ua::ReferenceTypeAttributes,
}

#[derive(Debug, Clone)]
pub struct DataTypeNode {
    pub requested_new_node_id: ua::NodeId,
    pub parent_node_id: ua::NodeId,
    pub reference_type_id: ua::NodeId,
    pub browse_name: ua::QualifiedName,
    pub attributes: ua::DataTypeAttributes,
}
