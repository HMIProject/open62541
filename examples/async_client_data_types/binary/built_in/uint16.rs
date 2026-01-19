use bytes::{Buf as _, Bytes};

use crate::{binary::StatelessBinaryReader, data_types::UInt16};

// [Part 6: 5.2.2.2 Integer](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.2)
impl StatelessBinaryReader for UInt16 {
    fn read(data: &mut Bytes) -> Self {
        Self(data.try_get_u16_le().unwrap())
    }
}
