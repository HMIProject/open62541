use bytes::{Buf as _, Bytes};

use crate::{binary::BinaryReader, data_types::Double};

// [Part 6: 5.2.2.3 Floating Point](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.3)
impl BinaryReader for Double {
    fn read(data: &mut Bytes) -> Self {
        Self(data.try_get_f64_le().unwrap())
    }
}
