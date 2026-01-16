use bytes::{Buf as _, Bytes};

use crate::{
    binary::BinaryReader,
    data_types::{Int16, Int32, Int64, SByte},
};

// [Part 6: 5.2.2.2 Integer](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.2)
impl BinaryReader for SByte {
    fn read(data: &mut Bytes) -> Self {
        Self(data.try_get_i8().unwrap())
    }
}

// [Part 6: 5.2.2.2 Integer](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.2)
impl BinaryReader for Int16 {
    fn read(data: &mut Bytes) -> Self {
        Self(data.try_get_i16_le().unwrap())
    }
}

// [Part 6: 5.2.2.2 Integer](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.2)
impl BinaryReader for Int32 {
    fn read(data: &mut Bytes) -> Self {
        Self(data.try_get_i32_le().unwrap())
    }
}

// [Part 6: 5.2.2.2 Integer](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.2)
impl BinaryReader for Int64 {
    fn read(data: &mut Bytes) -> Self {
        Self(data.try_get_i64_le().unwrap())
    }
}
