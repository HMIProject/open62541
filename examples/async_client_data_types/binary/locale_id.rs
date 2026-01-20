use bytes::Bytes;

use crate::{
    binary::BinaryReader,
    data_types::{LocaleId, String},
};

impl BinaryReader for LocaleId {
    fn read(data: &mut Bytes) -> Self {
        Self(String::read(data))
    }
}
