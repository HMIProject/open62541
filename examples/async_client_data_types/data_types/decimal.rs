use num_bigint::BigInt;

// [Part 3: 8.54 Decimal](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.54)
// [Part 6: 5.1.10 Decimal](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.10)
#[derive(Debug, Clone)]
pub struct Decimal(pub BigInt, pub i16);
