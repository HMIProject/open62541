use crate::{DataType as _, ua};

impl super::VariableAttributes {
    #[must_use]
    pub fn with_data_type(mut self, data_type: &ua::NodeId) -> Self {
        data_type.clone_into_raw(&mut self.0.dataType);
        self.0.specifiedAttributes |= ua::SpecifiedAttributes::DATATYPE.as_u32();
        self
    }

    #[must_use]
    pub const fn with_value_rank(mut self, rank: i32) -> Self {
        self.0.valueRank = rank;
        self.0.specifiedAttributes |= ua::SpecifiedAttributes::VALUERANK.as_u32();
        self
    }

    #[must_use]
    pub const fn with_access_level(mut self, access_level: &ua::AccessLevelType) -> Self {
        self.0.accessLevel = access_level.as_u8();
        self.0.specifiedAttributes |= ua::SpecifiedAttributes::ACCESSLEVEL.as_u32();
        self
    }

    #[must_use]
    pub fn with_array_dimensions(mut self, array_dimensions: ua::Array<ua::UInt32>) -> Self {
        drop(ua::Array::<ua::UInt32>::from_raw_parts(
            self.0.arrayDimensionsSize,
            self.0.arrayDimensions,
        ));
        (self.0.arrayDimensionsSize, self.0.arrayDimensions) = array_dimensions.into_raw_parts();
        self
    }
}
