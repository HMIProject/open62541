use std::{fmt, marker::PhantomData};

use crate::{ua, DataType, DataTypeExt};

/// Typed variant of [`ua::DataValue`].
//
// Do not derive trait implementations to avoid depending on the
// capabilities of the generic, marker type `T`. Instead we implement
// all applicable traits manually by delegating to the implementations
// for `ua::DataValue`.
pub struct DataValue<T> {
    data_value: ua::DataValue,
    _kind: PhantomData<T>,
}

impl<T> fmt::Debug for DataValue<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            data_value,
            _kind: _,
        } = self;
        data_value.fmt(f)
    }
}

impl<T> Clone for DataValue<T> {
    fn clone(&self) -> Self {
        let Self {
            data_value,
            _kind: _,
        } = self;
        Self {
            data_value: data_value.clone(),
            _kind: PhantomData,
        }
    }
}

impl<T> PartialEq for DataValue<T> {
    fn eq(&self, other: &Self) -> bool {
        let Self {
            data_value,
            _kind: _,
        } = self;
        data_value.eq(&other.data_value)
    }
}

impl<T> Eq for DataValue<T> {
    // `ua::DataValue` implements `Eq`.
    // TODO: Verify this at compile time.
}

impl<T> PartialOrd for DataValue<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for DataValue<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let Self {
            data_value,
            _kind: _,
        } = self;
        data_value.cmp(&other.data_value)
    }
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
