use std::marker::PhantomData;

use crate::{ua, DataType, DataTypeExt, Error, Result};

/// Typed variant of [`ua::DataValue`].
#[derive(Debug, Clone)]
pub struct DataValue<T> {
    data_value: ua::DataValue,
    _kind: PhantomData<T>,
}

impl<T: DataTypeExt> DataValue<T> {
    #[must_use]
    pub(crate) const fn new(data_value: ua::DataValue) -> Self {
        Self {
            data_value,
            _kind: PhantomData,
        }
    }

    #[expect(clippy::missing_errors_doc, reason = "deprecated method")]
    #[deprecated = "Use `Self::scalar_value()` instead."]
    pub fn value(&self) -> Result<&T>
    where
        T: DataType,
    {
        self.scalar_value()
    }

    /// Gets scalar value.
    ///
    /// # Errors
    ///
    /// This fails when the value is unset or not a scalar of the expected type.
    pub fn scalar_value(&self) -> Result<&T>
    where
        // `DataType` has transparent representation required for `as_scalar()`.
        T: DataType,
    {
        self.data_value
            .value()
            .ok_or(Error::internal("missing value"))?
            .as_scalar::<T>()
            .ok_or(Error::internal("unexpected data type"))
    }

    #[expect(clippy::missing_errors_doc, reason = "deprecated method")]
    #[deprecated = "Use `Self::into_scalar_value()` instead."]
    pub fn into_value(self) -> Result<T> {
        self.into_scalar_value()
    }

    /// Extracts scalar value.
    ///
    /// # Errors
    ///
    /// This fails when the value is unset or not a scalar of the expected type.
    pub fn into_scalar_value(self) -> Result<T> {
        let value = self
            .data_value
            .into_value()
            .ok_or(Error::internal("missing value"))?
            .into_scalar::<T::Inner>()
            .ok_or(Error::internal("unexpected data type"))?;

        Ok(T::from_inner(value))
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

    #[must_use]
    pub fn status(&self) -> Option<ua::StatusCode> {
        self.data_value.status()
    }
}

impl DataValue<ua::Variant> {
    /// Casts to specific value type.
    ///
    /// This adjusts the target type of `self`, casting the inner value to the specified data type
    /// when read with [`Self::value()`]. This should be used in situations when the expected type
    /// can be deduced from circumstances and typed data values can be returned for convenience.
    #[cfg_attr(not(feature = "tokio"), expect(dead_code, reason = "unused"))]
    pub(crate) fn cast<T: DataTypeExt>(self) -> DataValue<T> {
        let Self { data_value, _kind } = self;

        DataValue::new(data_value)
    }
}
