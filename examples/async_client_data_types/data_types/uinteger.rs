use crate::data_types::{Byte, UInt16, UInt32, UInt64};

// [Part 3: 8.33 UInteger](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.33)
// [Part 5: 12.2.9.10 UInteger](https://reference.opcfoundation.org/Core/Part5/v105/docs/12.2.9.10)
// [Part 6: 5.1.6 5.1.6 Number, Integer and UInteger](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.6)
#[derive(Debug, Clone, Copy)]
pub enum UInteger {
    Byte(Byte),
    UInt16(UInt16),
    UInt32(UInt32),
    UInt64(UInt64),
}

impl UInteger {
    pub fn is_zero(&self) -> bool {
        match self {
            Self::Byte(value) => value.is_zero(),
            Self::UInt16(value) => value.is_zero(),
            Self::UInt32(value) => value.is_zero(),
            Self::UInt64(value) => value.is_zero(),
        }
    }
}
