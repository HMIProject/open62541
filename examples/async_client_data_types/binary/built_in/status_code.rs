use bytes::Bytes;

use crate::{
    binary::BinaryReader,
    data_types::{StatusCode, UInt32},
};

// [Part 6: 5.2.2.11 StatusCode](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.11)
impl BinaryReader for StatusCode {
    fn read(data: &mut Bytes) -> Self {
        Self(UInt32::read(data).0)
    }
}
