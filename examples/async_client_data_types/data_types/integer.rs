use crate::data_types::{Int16, Int32, Int64, SByte};

// [Part 3: 8.24 Integer](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.24)
// [Part 5: 12.2.9.5 Integer](https://reference.opcfoundation.org/Core/Part5/v105/docs/12.2.9.5)
// [Part 6: 5.1.6 5.1.6 Number, Integer and UInteger](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.6)
#[derive(Debug, Clone, Copy)]
pub enum Integer {
    SByte(SByte),
    Int16(Int16),
    Int32(Int32),
    Int64(Int64),
}

impl Integer {
    pub fn is_zero(&self) -> bool {
        match self {
            Self::SByte(value) => value.is_zero(),
            Self::Int16(value) => value.is_zero(),
            Self::Int32(value) => value.is_zero(),
            Self::Int64(value) => value.is_zero(),
        }
    }
}
