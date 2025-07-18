use crate::{ua, DataType as _};

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
}
