use bytes::{Buf as _, Bytes};

use crate::{binary::StatelessBinaryReader, data_types::UInt32};

// [Part 6: 5.2.2.2 Integer](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.2)
impl StatelessBinaryReader for UInt32 {
    fn read(data: &mut Bytes) -> Self {
        Self(data.try_get_u32_le().unwrap())
    }
}
