use bytes::{Buf as _, Bytes};

use crate::{
    binary::BinaryReader,
    data_types::{Byte, UInt16, UInt32, UInt64},
};

// [Part 6: 5.2.2.2 Integer](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.2)
impl BinaryReader for Byte {
    fn read(data: &mut Bytes) -> Self {
        Self(data.try_get_u8().unwrap())
    }
}

// [Part 6: 5.2.2.2 Integer](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.2)
impl BinaryReader for UInt16 {
    fn read(data: &mut Bytes) -> Self {
        Self(data.try_get_u16_le().unwrap())
    }
}

// [Part 6: 5.2.2.2 Integer](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.2)
impl BinaryReader for UInt32 {
    fn read(data: &mut Bytes) -> Self {
        Self(data.try_get_u32_le().unwrap())
    }
}

// [Part 6: 5.2.2.2 Integer](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.2)
impl BinaryReader for UInt64 {
    fn read(data: &mut Bytes) -> Self {
        Self(data.try_get_u64_le().unwrap())
    }
}
