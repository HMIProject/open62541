use bytes::Bytes;

use crate::{
    binary::StatelessBinaryReader,
    data_types::{LocaleId, String},
};

impl StatelessBinaryReader for LocaleId {
    fn read(data: &mut Bytes) -> Self {
        Self(String::read(data))
    }
}
