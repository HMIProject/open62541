pub use self::{int16::Int16, int32::Int32, int64::Int64, sbyte::SByte};

// [Part 3: 8.24 Integer](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.24)
// [Part 5: 12.2.9.5 Integer](https://reference.opcfoundation.org/Core/Part5/v105/docs/12.2.9.5)
// [Part 6: 5.1.6 5.1.6 Number, Integer and UInteger](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.6)
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

mod sbyte {
    // [Part 3: 8.17 Sbyte](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.17)
    pub struct SByte(pub i8);

    impl SByte {
        pub fn is_zero(&self) -> bool {
            self.0 == 0
        }
    }
}

mod int16 {
    // [Part 3: 8.25 Int16](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.25)
    pub struct Int16(pub i16);

    impl Int16 {
        pub fn is_zero(&self) -> bool {
            self.0 == 0
        }
    }
}

mod int32 {
    // [Part 3: 8.26 Int32](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.32)
    pub struct Int32(pub i32);

    impl Int32 {
        pub fn is_zero(&self) -> bool {
            self.0 == 0
        }
    }
}

mod int64 {
    // [Part 3: 8.27 Int64](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.37)
    pub struct Int64(pub i64);

    impl Int64 {
        pub fn is_zero(&self) -> bool {
            self.0 == 0
        }
    }
}
