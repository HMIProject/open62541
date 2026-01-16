use bytes::Bytes;

use crate::{
    binary::BinaryReader,
    data_types::{Byte, LocaleId, LocalizedText, String},
};

// [Part 6: 5.2.2.14 LocalizedText](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.14)
impl BinaryReader for LocalizedText {
    fn read(data: &mut Bytes) -> Self {
        let encoding_mask = Byte::read(data);
        let locale = (encoding_mask.0 & 0x01 != 0x00)
            .then(|| LocaleId::read(data))
            .unwrap_or_else(LocaleId::null);
        let text = (encoding_mask.0 & 0x02 != 0x00)
            .then(|| String::read(data))
            .unwrap_or_else(String::null);
        Self(locale, text)
    }
}
