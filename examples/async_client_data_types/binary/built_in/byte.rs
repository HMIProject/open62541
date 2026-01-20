use bytes::{Buf as _, Bytes};

use crate::{binary::BinaryReader, data_types::Byte};

// [Part 6: 5.2.2.2 Integer](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.2)
impl BinaryReader for Byte {
    fn read(data: &mut Bytes) -> Self {
        Self(data.try_get_u8().unwrap())
    }
}
