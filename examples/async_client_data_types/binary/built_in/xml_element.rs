use std::string::String as StdString;

use bytes::Bytes;

use crate::{
    binary::BinaryReader,
    data_types::{ByteString, XmlElement},
};

// [Part 6: 5.2.2.8 XmlElement (Deprecated)](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.8)
impl BinaryReader for XmlElement {
    fn read(data: &mut Bytes) -> Self {
        let Some(string) = ByteString::read(data).0 else {
            return Self(None);
        };

        let string = StdString::from_utf8(string.into_vec()).unwrap();
        Self(Some(string.into_boxed_str()))
    }
}
