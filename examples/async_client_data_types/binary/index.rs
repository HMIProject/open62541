use bytes::Bytes;

use crate::{
    binary::StatelessBinaryReader,
    data_types::{Index, UInt32},
};

impl StatelessBinaryReader for Index {
    fn read(data: &mut Bytes) -> Self {
        Self(UInt32::read(data).0)
    }
}
