use crate::ua::{self, Attributes};

#[derive(Debug, Clone)]
pub struct Node {
    pub requested_new_node_id: ua::NodeId,
    pub parent_node_id: ua::NodeId,
    pub reference_type_id: ua::NodeId,
    pub browse_name: ua::QualifiedName,
    pub type_definition: Option<ua::NodeId>,
    pub attributes: Attributes,
}

impl Node {
    pub fn get_type_definition(&self) -> ua::NodeId {
        match self.attributes {
            Attributes::DataType(_) => ua::NodeId::null(),
            Attributes::Object(_) => self.type_definition.as_ref().expect("Type definition for ObjectNode must be specified!").clone(),
            Attributes::ObjectType(_) => ua::NodeId::null(),
            Attributes::ReferenceType(_) => ua::NodeId::null(),
            Attributes::Variable(_) => self.type_definition.as_ref().expect("Type definition for VariableNode must be specified!").clone(),
            Attributes::VariableType(_) => self.type_definition.as_ref().expect("Type definition for VariableTypeNode must be specified!").clone(),
            Attributes::View(_) => ua::NodeId::null(),
        }
    }
}
