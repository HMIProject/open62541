mod integer;
mod uinteger;

pub use self::{
    decimal::Decimal,
    double::Double,
    float::Float,
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

mod float {
    // [Part 3: 8.15 Float](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.15)
    pub struct Float(pub f32);
}

mod double {
    // [Part 3: 8.12 Double](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.12)
    pub struct Double(pub f64);
}

mod decimal {
    use num_bigint::BigInt;

    // [Part 3: 8.54 Decimal](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.54)
    // [Part 6: 5.1.10 Decimal](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.10)
    pub struct Decimal(pub BigInt, pub i16);
}
