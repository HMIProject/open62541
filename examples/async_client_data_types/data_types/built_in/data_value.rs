use crate::data_types::{DateTime, StatusCode, Variant};

// [Part 4: 7.11 DataValue](https://reference.opcfoundation.org/Core/Part4/v105/docs/7.11)
// [Part 5: 12.3.5 DataValue](https://reference.opcfoundation.org/Core/Part5/v105/docs/12.3.5)
// [Part 6: 5.2.2.17 DataValue](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.17)
#[derive(Debug, Clone)]
pub struct DataValue {
    pub value: Variant,
    pub status: StatusCode,
    pub source_timestamp: DateTime,
    pub source_picoseconds: Option<u16>,
    pub server_timestamp: DateTime,
    pub server_picoseconds: Option<u16>,
}
