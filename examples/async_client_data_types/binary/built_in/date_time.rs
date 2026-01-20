use bytes::Bytes;

use crate::{
    binary::BinaryReader,
    data_types::{DateTime, Int64},
};

// [Part 6: 5.2.2.5 DateTime](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.5)
impl BinaryReader for DateTime {
    fn read(data: &mut Bytes) -> Self {
        let value = Int64::read(data);

        if value.0 == 0 || value.0 < Self::min_value().0 {
            Self::min_value()
        } else if value.0 == i64::MAX || value.0 > Self::max_value().0 {
            Self::max_value()
        } else {
            Self(value.0)
        }
    }
}
