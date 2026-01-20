use bytes::{Buf as _, Bytes};

use crate::{binary::BinaryReader, data_types::SByte};

// [Part 6: 5.2.2.2 Integer](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.2)
impl BinaryReader for SByte {
    fn read(data: &mut Bytes) -> Self {
        Self(data.try_get_i8().unwrap())
    }
}
