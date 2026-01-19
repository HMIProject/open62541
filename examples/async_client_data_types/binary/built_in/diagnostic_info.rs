use crate::{
    binary::StatelessBinaryReader,
    data_types::{Byte, DiagnosticInfo, Int32, StatusCode, String},
};

// [Part 6: 5.2.2.12 DiagnosticInfo](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.12)
impl StatelessBinaryReader for DiagnosticInfo {
    fn read(data: &mut bytes::Bytes) -> Self {
        let encoding_mask = Byte::read(data);

        let symbolic_id = (encoding_mask.0 & 0x01) != 0;
        let namespace = (encoding_mask.0 & 0x02) != 0;
        let localized_text = (encoding_mask.0 & 0x04) != 0;
        let locale = (encoding_mask.0 & 0x08) != 0;
        let additional_info = (encoding_mask.0 & 0x10) != 0;
        let inner_status_code = (encoding_mask.0 & 0x20) != 0;
        let inner_diagnostic_info = (encoding_mask.0 & 0x40) != 0;

        let symbolic_id = symbolic_id.then(|| Int32::read(data).0);
        let namespace_uri = namespace.then(|| Int32::read(data).0);
        let locale = locale.then(|| Int32::read(data).0);
        let localized_text = localized_text.then(|| Int32::read(data).0);
        let additional_info = additional_info.then(|| String::read(data));
        let inner_status_code = inner_status_code.then(|| StatusCode::read(data));
        let inner_diagnostic_info =
            inner_diagnostic_info.then(|| Box::new(DiagnosticInfo::read(data)));

        Self {
            namespace_uri,
            symbolic_id,
            locale,
            localized_text,
            additional_info,
            inner_status_code,
            inner_diagnostic_info,
        }
    }
}
