use crate::data_types::{LocalizedText, NodeId, String};

// [Part 3: 8.51 StructureField](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.51)
// [Part 5: 12.2.12.10 StructureField](https://reference.opcfoundation.org/Core/Part5/v105/docs/12.2.12.10)
#[derive(Debug, Clone)]
pub struct StructureField {
    pub name: String,
    pub description: LocalizedText,
    pub data_type: NodeId,
    pub value_rank: i32,
    pub array_dimensions: Box<[u32]>,
    pub max_string_length: u32,
    pub is_optional: bool,
}

impl StructureField {
    #[must_use]
    pub fn is_scalar(&self) -> bool {
        self.value_rank == -1
    }

    #[must_use]
    pub fn is_array(&self) -> bool {
        self.value_rank >= 1
    }

    #[must_use]
    pub fn is_one_dimensional_array(&self) -> bool {
        self.is_array() && self.array_dimensions.len() == 1
    }

    #[must_use]
    pub fn is_multi_dimensional_array(&self) -> bool {
        self.is_array() && self.array_dimensions.len() >= 2
    }
}
