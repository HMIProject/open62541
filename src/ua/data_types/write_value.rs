use crate::{DataType as _, ua};

crate::data_type!(WriteValue);

impl WriteValue {
    #[must_use]
    pub fn with_node_id(mut self, node_id: &ua::NodeId) -> Self {
        node_id.clone_into_raw(&mut self.0.nodeId);
        self
    }

    #[must_use]
    pub fn with_attribute_id(mut self, attribute_id: &ua::AttributeId) -> Self {
        self.0.attributeId = attribute_id.as_u32();
        self
    }

    #[must_use]
    pub fn with_value(mut self, value: &ua::DataValue) -> Self {
        value.clone_into_raw(&mut self.0.value);
        self
    }
}
