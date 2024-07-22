use crate::ua;

pub trait AsNodeAttributes {
    fn as_node_attributes(&self) -> &ua::NodeAttributes;
}
