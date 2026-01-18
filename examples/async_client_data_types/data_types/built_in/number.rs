mod integer;
mod uinteger;

use num_bigint::BigInt;

pub use self::{
    integer::{Int16, Int32, Int64, Integer, SByte},
    uinteger::{Byte, UInt16, UInt32, UInt64, UInteger},
};

// [Part 3: 8.30 Number](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.30)
// [Part 5: 12.2.9 Number](https://reference.opcfoundation.org/Core/Part5/v105/docs/12.2.9)
// [Part 6: 5.1.6 5.1.6 Number, Integer and UInteger](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.6)
pub enum Number {
    Integer(Integer),
    UInteger(UInteger),
    Float(Float),
    Double(Double),
    Decimal(Decimal),
}

// [Part 3: 8.15 Float](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.15)
pub struct Float(pub f32);

// [Part 3: 8.12 Double](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.12)
pub struct Double(pub f64);

// [Part 3: 8.54 Decimal](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.54)
// [Part 6: 5.1.10 Decimal](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.10)
pub struct Decimal(pub BigInt, pub i16);

// [Part 4: 7.18 Index](https://reference.opcfoundation.org/Core/Part4/v105/docs/7.18)
pub struct Index(pub u32);

impl Index {
    pub fn zero() -> Self {
        Self(0)
    }
}
