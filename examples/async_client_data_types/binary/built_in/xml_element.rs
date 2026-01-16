use std::string::String as StdString;

use bytes::Bytes;

use crate::{
    binary::BinaryReader,
    data_types::{ByteString, XmlElement},
};

// [Part 6: 5.2.2.8 XmlElement (Deprecated)](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.8)
impl BinaryReader for XmlElement {
    fn read(data: &mut Bytes) -> Self {
        let string = ByteString::read(data);
        Self(string.0.map(|string| StdString::from_utf8(string).unwrap()))
    }
}
