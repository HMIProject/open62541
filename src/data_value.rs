use crate::{ua, DataType, Error, Result};

/// Typed variant of [`ua::DataValue`].
#[derive(Debug, Clone)]
pub struct DataValue<T> {
    value: Option<T>,
    source_timestamp: Option<ua::DateTime>,
    server_timestamp: Option<ua::DateTime>,
    source_picoseconds: Option<u16>,
    server_picoseconds: Option<u16>,
}

impl<T: DataType> DataValue<T> {
    pub(crate) fn new(data_value: &ua::DataValue) -> Result<Self> {
        let value = data_value
            .value()
            .map(|value| {
                value
                    .to_scalar::<T>()
                    .ok_or(Error::internal("unexpected data type"))
            })
            .transpose()?;

        Ok(Self {
            value,
            source_timestamp: data_value.source_timestamp().cloned(),
            server_timestamp: data_value.server_timestamp().cloned(),
            source_picoseconds: data_value.source_picoseconds(),
            server_picoseconds: data_value.server_picoseconds(),
        })
    }

    #[must_use]
    pub const fn value(&self) -> Option<&T> {
        self.value.as_ref()
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
    pub(crate) fn cast<T: DataType>(self) -> Result<DataValue<T>> {
        let Self {
            value,
            source_timestamp,
            server_timestamp,
            source_picoseconds,
            server_picoseconds,
        } = self;

        let value = value
            .map(|value| {
                value
                    .to_scalar::<T>()
                    .ok_or(Error::internal("unexpected data type"))
            })
            .transpose()?;

        Ok(DataValue {
            value,
            source_timestamp,
            server_timestamp,
            source_picoseconds,
            server_picoseconds,
        })
    }
}
