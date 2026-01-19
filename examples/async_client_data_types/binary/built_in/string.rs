use std::string::String as StdString;

use bytes::{Buf as _, Bytes};

use crate::{
    binary::StatelessBinaryReader,
    data_types::{Int32, String},
};

// [Part 6: 5.2.2.4 String](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.4)
impl StatelessBinaryReader for String {
    fn read(data: &mut Bytes) -> Self {
        let length = Int32::read(data);
        if length.0 == -1 {
            return Self(None);
        }
        let length = usize::try_from(length.0).unwrap();

        let mut bytes = vec![0; length];
        data.try_copy_to_slice(&mut bytes).unwrap();

        let string = StdString::from_utf8(bytes).unwrap();
        Self(Some(string.into_boxed_str()))
    }
}
