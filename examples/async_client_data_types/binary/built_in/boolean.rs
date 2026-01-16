use bytes::{Buf as _, Bytes};

use crate::{binary::BinaryReader, data_types::Boolean};

// [Part 6: 5.2.2.1 Boolean](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.1)
impl BinaryReader for Boolean {
    fn read(data: &mut Bytes) -> Self {
        Self(data.try_get_i8().unwrap() != 0)
    }
}
