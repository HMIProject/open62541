use crate::data_types::{Decimal, Double, Float, Integer, UInteger};

// [Part 3: 8.30 Number](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.30)
// [Part 5: 12.2.9 Number](https://reference.opcfoundation.org/Core/Part5/v105/docs/12.2.9)
// [Part 6: 5.1.6 5.1.6 Number, Integer and UInteger](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.6)
#[derive(Debug, Clone)]
pub enum Number {
    Integer(Integer),
    UInteger(UInteger),
    Float(Float),
    Double(Double),
    Decimal(Decimal),
}
