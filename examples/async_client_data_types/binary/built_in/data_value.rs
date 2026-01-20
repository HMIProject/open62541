use bytes::{Buf, Bytes};

use crate::{
    binary::BinaryReader,
    data_types::{DataValue, DateTime, StatusCode, UInt16, Variant},
};

// [Part 6: 5.2.2.17 DataValue](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.17)
impl BinaryReader for DataValue {
    fn read(data: &mut Bytes) -> Self {
        let encoding_mask = data.try_get_u8().unwrap();

        let has_value = (encoding_mask & 0x01) != 0;
        let has_status_code = (encoding_mask & 0x02) != 0;
        let has_source_timestamp = (encoding_mask & 0x04) != 0;
        let has_server_timestamp = (encoding_mask & 0x08) != 0;
        let has_source_picoseconds = (encoding_mask & 0x10) != 0;
        let has_server_picoseconds = (encoding_mask & 0x20) != 0;

        let value = has_value
            .then(|| Variant::read(data))
            .unwrap_or_else(Variant::null);
        let status_code = has_status_code
            .then(|| StatusCode::read(data))
            .unwrap_or_else(StatusCode::good);
        let source_timestamp = has_source_timestamp
            .then(|| DateTime::read(data))
            .unwrap_or_else(DateTime::min_value);
        let source_picoseconds = has_source_picoseconds.then(|| UInt16::read(data).0);
        let server_timestamp = has_server_timestamp
            .then(|| DateTime::read(data))
            .unwrap_or_else(DateTime::min_value);
        let server_picoseconds = has_server_picoseconds.then(|| UInt16::read(data).0);

        let source_picoseconds = has_source_timestamp.then_some(source_picoseconds).flatten();
        let server_picoseconds = has_server_timestamp.then_some(server_picoseconds).flatten();

        Self {
            value,
            status: status_code,
            source_timestamp,
            source_picoseconds,
            server_timestamp,
            server_picoseconds,
        }
    }
}
