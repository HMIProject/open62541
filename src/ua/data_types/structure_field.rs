use crate::{DataType, ua};

crate::data_type!(StructureField);

impl StructureField {
    #[must_use]
    pub fn name(&self) -> &ua::String {
        ua::String::raw_ref(&self.0.name)
    }

    #[must_use]
    pub fn description(&self) -> &ua::LocalizedText {
        ua::LocalizedText::raw_ref(&self.0.description)
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
    pub fn array_dimensions(&self) -> Option<Vec<u32>> {
        // TODO: Adjust signature to return non-owned value instead.
        ua::Array::<ua::UInt32>::from_raw_parts(self.0.arrayDimensionsSize, self.0.arrayDimensions)
            .map(|array_dimensions| array_dimensions.iter().map(ua::UInt32::value).collect())
    }

    #[must_use]
    pub const fn max_string_length(&self) -> u32 {
        self.0.maxStringLength
    }

    #[must_use]
    pub const fn is_optional(&self) -> bool {
        self.0.isOptional
    }
}
