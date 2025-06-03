use std::marker::PhantomData;

use crate::{ua, DataType, DataTypeExt};

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

    /// Gets value.
    ///
    /// This returns `None` when the value is unset.
    #[must_use]
    pub fn value(&self) -> Option<&ua::Variant> {
        self.data_value.value()
    }

    /// Extracts value.
    ///
    /// This returns `None` when the value is unset.
    #[must_use]
    pub fn into_value(self) -> Option<ua::Variant> {
        self.data_value.into_value()
    }

    /// Gets scalar value.
    ///
    /// This returns `None` when the value is unset or not a scalar of the given type. The same can
    /// be achieved in two steps with [`Self::value()`] and [`ua::Variant::as_scalar()`].
    pub fn scalar_value(&self) -> Option<&T>
    where
        // `DataType` has transparent representation required for `as_scalar()`.
        T: DataType,
    {
        self.value().and_then(ua::Variant::as_scalar::<T>)
    }

    /// Extracts scalar value.
    ///
    /// This returns `None` when the value is unset or not a scalar of the given type. The same can
    /// be achieved in two steps with [`Self::into_value()`] and [`ua::Variant::into_scalar()`].
    pub fn into_scalar_value(self) -> Option<T> {
        self.into_value()
            .and_then(ua::Variant::into_scalar::<T::Inner>)
            .map(T::from_inner)
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
