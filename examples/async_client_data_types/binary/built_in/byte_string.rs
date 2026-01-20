use bytes::{Buf as _, Bytes};

use crate::{
    binary::BinaryReader,
    data_types::{ByteString, Int32},
};

// [Part 6: 5.2.2.7 ByteString](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.7)
impl BinaryReader for ByteString {
    fn read(data: &mut Bytes) -> Self {
        let length = Int32::read(data);
        if length.0 == -1 {
            return Self(None);
        }
        let length = usize::try_from(length.0).unwrap();

        let mut bytes = vec![0; length];
        data.try_copy_to_slice(&mut bytes).unwrap();

        Self(Some(bytes.into_boxed_slice()))
    }
}
