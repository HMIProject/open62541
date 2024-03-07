use crate::{ua, DataType};

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
        ua::Array::from_raw_parts(self.0.arrayDimensions, self.0.arrayDimensionsSize)
    }

    #[must_use]
    pub fn description(&self) -> &ua::LocalizedText {
        ua::LocalizedText::raw_ref(&self.0.description)
    }
}
