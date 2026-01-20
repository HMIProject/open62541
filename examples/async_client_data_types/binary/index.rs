use bytes::Bytes;

use crate::{
    binary::BinaryReader,
    data_types::{Index, UInt32},
};

impl BinaryReader for Index {
    fn read(data: &mut Bytes) -> Self {
        Self(UInt32::read(data).0)
    }
}
