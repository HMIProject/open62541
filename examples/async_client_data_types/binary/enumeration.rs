use crate::{
    binary::StatelessBinaryReader,
    data_types::{Enumeration, Int32},
};

// [Part 6: 5.2.4 Enumerations](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.4)
impl StatelessBinaryReader for Enumeration {
    fn read(data: &mut bytes::Bytes) -> Self {
        Self(Int32::read(data).0)
    }
}
