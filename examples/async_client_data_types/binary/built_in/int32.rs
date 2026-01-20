use bytes::{Buf as _, Bytes};

use crate::{binary::BinaryReader, data_types::Int32};

// [Part 6: 5.2.2.2 Integer](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.2)
impl BinaryReader for Int32 {
    fn read(data: &mut Bytes) -> Self {
        Self(data.try_get_i32_le().unwrap())
    }
}
