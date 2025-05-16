use crate::{ua, DataType, Error, Result};

/// Typed variant of [`ua::DataValue`].
#[derive(Debug, Clone)]
pub struct DataValue<T> {
    value: T,
    source_timestamp: Option<ua::DateTime>,
    server_timestamp: Option<ua::DateTime>,
    source_picoseconds: Option<u16>,
    server_picoseconds: Option<u16>,
}

impl<T: DataType> DataValue<T> {
    pub(crate) fn new(data_value: &ua::DataValue) -> Result<Self> {
        // Verify that data value is valid before accessing value. The OPC UA specification requires
        // us to do so. The status code may be omitted, in which case it is treated as valid data.
        Error::verify_good(&data_value.status().unwrap_or(ua::StatusCode::GOOD))?;

        // When the status code indicates a good data value, the value is expected to be set.
        let value = data_value
            .value()
            .ok_or(Error::Internal("missing value"))?
            .to_scalar::<T>()
            .ok_or(Error::internal("unexpected data type"))?;

        Ok(Self {
            value,
            source_timestamp: data_value.source_timestamp().cloned(),
            server_timestamp: data_value.server_timestamp().cloned(),
            source_picoseconds: data_value.source_picoseconds(),
            server_picoseconds: data_value.server_picoseconds(),
        })
    }

    #[must_use]
    pub const fn value(&self) -> &T {
        &self.value
    }

    #[must_use]
    pub fn into_value(self) -> T {
        self.value
    }

    #[must_use]
    pub const fn source_timestamp(&self) -> Option<&ua::DateTime> {
        self.source_timestamp.as_ref()
    }

    #[must_use]
    pub const fn server_timestamp(&self) -> Option<&ua::DateTime> {
        self.server_timestamp.as_ref()
    }

    #[must_use]
    pub const fn source_picoseconds(&self) -> Option<u16> {
        self.source_picoseconds
    }

    #[must_use]
    pub const fn server_picoseconds(&self) -> Option<u16> {
        self.server_picoseconds
    }
}

impl DataValue<ua::Variant> {
    /// Cast to specific value type.
    ///
    /// This consumes `self` and casts the inner value to the specified data type. This should be
    /// used in situation where the expected type can be deduced from circumstances and unwrapped
    /// data values are needed for convenience. This always expects a scalar value.
    #[cfg_attr(not(feature = "tokio"), expect(dead_code, reason = "unused"))]
    pub(crate) fn into_scalar<T: DataType>(self) -> Result<DataValue<T>> {
        let Self {
            value,
            source_timestamp,
            server_timestamp,
            source_picoseconds,
            server_picoseconds,
        } = self;

        let value = value
            .to_scalar::<T>()
            .ok_or(Error::internal("unexpected data type"))?;

        Ok(DataValue {
            value,
            source_timestamp,
            server_timestamp,
            source_picoseconds,
            server_picoseconds,
        })
    }
}
