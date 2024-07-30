use crate::{ua, DataType, ValueType};

crate::data_type!(Argument);

impl Argument {
    #[must_use]
    pub fn name(&self) -> &ua::String {
        ua::String::raw_ref(&self.0.name)
    }

    #[must_use]
    pub fn data_type(&self) -> &ua::NodeId {
        ua::NodeId::raw_ref(&self.0.dataType)
    }

    #[must_use]
    pub const fn value_rank(&self) -> i32 {
        self.0.valueRank
    }

    #[must_use]
    pub fn array_dimensions(&self) -> Option<ua::Array<ua::UInt32>> {
        // TODO: Adjust signature to return non-owned value instead.
        ua::Array::from_raw_parts(self.0.arrayDimensionsSize, self.0.arrayDimensions)
    }

    #[must_use]
    pub fn description(&self) -> &ua::LocalizedText {
        ua::LocalizedText::raw_ref(&self.0.description)
    }

    #[must_use]
    pub fn value_type(&self) -> ValueType {
        ValueType::from_data_type(self.data_type())
    }

    pub fn with_name(mut self, name: ua::String) -> Self {
        name.move_into_raw(&mut self.0.name);
        self
    }

    pub fn with_data_type(mut self, data_type: ua::NodeId) -> Self {
        data_type.move_into_raw(&mut self.0.dataType);
        self
    }

    pub fn with_value_rank(mut self, value_rank: i32) -> Self {
        self.0.valueRank = value_rank;
        self
    }

    pub fn with_description(mut self, description: ua::LocalizedText) -> Self {
        description.move_into_raw(&mut self.0.description);
        self
    }
}
