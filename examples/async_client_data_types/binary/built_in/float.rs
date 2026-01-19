use bytes::{Buf as _, Bytes};

use crate::{binary::StatelessBinaryReader, data_types::Float};

// [Part 6: 5.2.2.3 Floating Point](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.3)
impl StatelessBinaryReader for Float {
    fn read(data: &mut Bytes) -> Self {
        Self(data.try_get_f32_le().unwrap())
    }
}
