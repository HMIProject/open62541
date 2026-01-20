use bytes::{Buf as _, Bytes};
use num_bigint::BigInt;

use crate::{
    binary::BinaryReader,
    data_types::{Byte, Decimal, Int16, Int32, NodeId},
};

// [Part 6: 5.1.10 Decimal](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.10)
// [Part 6: 5.2.3 Decimal](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.3)
impl BinaryReader for Decimal {
    fn read(data: &mut Bytes) -> Self {
        let type_id = NodeId::read(data);
        assert!(matches!(type_id, NodeId::Numeric(0, identifier) if identifier == 50));
        let encoding = Byte::read(data);
        assert!(encoding.0 == 1);
        let length = Int32::read(data);
        assert!(length.0 > 2);
        let scale = Int16::read(data);
        let value_length = usize::try_from(length.0.checked_sub(2).unwrap()).unwrap();
        let mut value = vec![0; value_length];
        data.try_copy_to_slice(&mut value).unwrap();
        let value = BigInt::from_signed_bytes_le(&value);
        Self(value, scale.0)
    }
}
