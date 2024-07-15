use crate::{
    ua::{self, Attributes},
    NodeContext,
};

pub struct Node {
    pub node_id: ua::NodeId,
    pub parent_node_id: ua::NodeId,
    pub reference_type_id: ua::NodeId,
    pub browse_name: ua::QualifiedName,
    pub type_definition: Option<ua::NodeId>,
    pub node_context: Option<NodeContext>,
    pub attributes: Attributes,
}

impl Node {
    /// # Panics
    ///
    /// This may panic if `Object`, `Variable` or `VariableType` Attributes are used but no `typeDefinition` is specified.
    #[must_use]
    pub fn get_type_definition(&self) -> ua::NodeId {
        match self.attributes {
            Attributes::DataType(_)
            | Attributes::ObjectType(_)
            | Attributes::ReferenceType(_)
            | Attributes::View(_) => ua::NodeId::null(),
            Attributes::Object(_) => self
                .type_definition
                .as_ref()
                .expect("Type definition for ObjectNode must be specified!")
                .clone(),
            Attributes::Variable(_) => self
                .type_definition
                .as_ref()
                .expect("Type definition for VariableNode must be specified!")
                .clone(),
            Attributes::VariableType(_) => self
                .type_definition
                .as_ref()
                .expect("Type definition for VariableTypeNode must be specified!")
                .clone(),
        }
    }
}
