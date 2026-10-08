use crate::{DataType, ValueType, ua};

crate::data_type!(Argument);
crate::member_accessors!(Argument {
    dataType: &ua::NodeId,
    name: &ua::String,
    description: &ua::LocalizedText,
    valueRank: i32,
    arrayDimensions: [ua::UInt32]
});

impl Argument {
    #[must_use]
    pub fn with_name(mut self, name: &ua::String) -> Self {
        name.clone_into_raw(&mut self.0.name);
        self
    }

    #[must_use]
    pub fn with_data_type(mut self, data_type: &ua::NodeId) -> Self {
        data_type.clone_into_raw(&mut self.0.dataType);
        self
    }

    #[must_use]
    pub const fn with_value_rank(mut self, value_rank: i32) -> Self {
        self.0.valueRank = value_rank;
        self
    }

    #[must_use]
    pub fn with_description(mut self, description: &ua::LocalizedText) -> Self {
        description.clone_into_raw(&mut self.0.description);
        self
    }

    #[must_use]
    pub fn value_type(&self) -> ValueType {
        ValueType::from_data_type(self.data_type())
    }
}
