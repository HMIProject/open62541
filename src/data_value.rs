use std::marker::PhantomData;

use crate::{ua, DataType, Error, Result};

/// Typed variant of [`ua::DataValue`].
#[derive(Debug, Clone)]
pub struct DataValue<T> {
    data_value: ua::DataValue,
    _kind: PhantomData<T>,
}

impl<T: DataType> DataValue<T> {
    #[must_use]
    pub(crate) const fn new(data_value: ua::DataValue) -> Self {
        Self {
            data_value,
            _kind: PhantomData,
        }
    }

    /// Gets scalar value.
    ///
    /// # Errors
    ///
    /// This fails when the value is unset or not a scalar of the expected type.
    pub fn value(&self) -> Result<&T> {
        self.data_value
            .value()
            .ok_or(Error::internal("missing value"))?
            .as_scalar::<T>()
            .ok_or(Error::internal("unexpected data type"))
    }

    /// Extracts scalar value.
    ///
    /// # Errors
    ///
    /// This fails when the value is unset or not a scalar of the expected type.
    pub fn into_value(self) -> Result<T> {
        self.data_value
            .into_value()
            .ok_or(Error::internal("missing value"))?
            .into_scalar::<T>()
            .ok_or(Error::internal("unexpected data type"))
    }

    #[must_use]
    pub fn source_timestamp(&self) -> Option<&ua::DateTime> {
        self.data_value.source_timestamp()
    }

    #[must_use]
    pub fn server_timestamp(&self) -> Option<&ua::DateTime> {
        self.data_value.server_timestamp()
    }

    #[must_use]
    pub fn source_picoseconds(&self) -> Option<u16> {
        self.data_value.source_picoseconds()
    }

    #[must_use]
    pub fn server_picoseconds(&self) -> Option<u16> {
        self.data_value.server_picoseconds()
    }
}

impl DataValue<ua::Variant> {
    /// Casts to specific value type.
    ///
    /// This adjusts the target type of `self`, casting the inner value to the specified data type
    /// when read with [`Self::value()`]. This should be used in situations when the expected type
    /// can be deduced from circumstances and typed data values can be returned for convenience.
    #[cfg_attr(not(feature = "tokio"), expect(dead_code, reason = "unused"))]
    pub(crate) fn cast<T: DataType>(self) -> DataValue<T> {
        let Self { data_value, _kind } = self;

        DataValue {
            data_value,
            _kind: PhantomData,
        }
    }
}
