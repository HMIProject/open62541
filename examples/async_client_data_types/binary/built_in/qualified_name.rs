use bytes::Bytes;

use crate::{
    binary::StatelessBinaryReader,
    data_types::{QualifiedName, String, UInt16},
};

// [Part 6: 5.2.2.13 QualifiedName](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.13)
impl StatelessBinaryReader for QualifiedName {
    fn read(data: &mut Bytes) -> Self {
        let namespace_index = UInt16::read(data);
        let name = String::read(data);

        Self(namespace_index.0, name)
    }
}
