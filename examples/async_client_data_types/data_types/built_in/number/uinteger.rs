pub use self::{byte::Byte, uint16::UInt16, uint32::UInt32, uint64::UInt64};

// [Part 3: 8.33 UInteger](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.33)
// [Part 5: 12.2.9.10 UInteger](https://reference.opcfoundation.org/Core/Part5/v105/docs/12.2.9.10)
// [Part 6: 5.1.6 5.1.6 Number, Integer and UInteger](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.6)
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

mod byte {
    // [Part 3: 8.9 Byte](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.9)
    pub struct Byte(pub u8);

    impl Byte {
        pub fn is_zero(&self) -> bool {
            self.0 == 0
        }
    }
}

mod uint16 {
    // [Part 3: 8.34 UInt16](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.34)
    pub struct UInt16(pub u16);

    impl UInt16 {
        pub fn is_zero(&self) -> bool {
            self.0 == 0
        }
    }
}

mod uint32 {
    // [Part 3: 8.35 UInt32](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.35)
    pub struct UInt32(pub u32);

    impl UInt32 {
        pub fn is_zero(&self) -> bool {
            self.0 == 0
        }
    }
}

mod uint64 {
    // [Part 3: 8.36 UInt64](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.36)
    pub struct UInt64(pub u64);

    impl UInt64 {
        pub fn is_zero(&self) -> bool {
            self.0 == 0
        }
    }
}
