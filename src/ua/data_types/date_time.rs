crate::data_type!(DateTime);

impl DateTime {
    /// Minimum value.
    ///
    /// See also: <https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.5>
    pub const MIN: Self = Self(0);

    /// Maximum value.
    ///
    /// See also: <https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.5>
    pub const MAX: Self = Self(2_650_467_743_990_000_000);

    /// Minimum UTC timestamp.
    ///
    /// See also: <https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.5>
    #[cfg(feature = "time")]
    pub const MIN_UTC: time::OffsetDateTime = time::macros::datetime!(1601-01-01 00:00:00 UTC);

    /// Maximum UTC timestamp.
    ///
    /// See also: <https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.5>
    #[cfg(feature = "time")]
    pub const MAX_UTC: time::OffsetDateTime = time::macros::datetime!(9999-12-31 23:59:59 UTC);

    /// Converts the value to an UTC timestamp.
    ///
    /// The values [`MIN`](Self::MIN) and [`MAX`](Self::MAX) are converted as is and not
    /// mapped to the corresponding limits of [`time::OffsetDateTime`].
    #[cfg(feature = "time")]
    #[must_use]
    pub fn to_utc(&self) -> Option<time::OffsetDateTime> {
        use open62541_sys::{UA_DATETIME_UNIX_EPOCH, UA_DATETIME_USEC};

        // TODO: How to handle values that are out of range?
        //debug_assert!(*self >= Self::MIN, "DateTime exceeds minimum value");
        //debug_assert!(*self <= Self::MAX, "DateTime exceeds maximum value");

        // OPC UA encodes `DateTime` as Windows file time: a 64-bit value that represents the number
        // of 100-nanosecond intervals that have elapsed since 12:00 A.M. January 1, 1601 (UTC).
        let ticks_ua = i128::from(self.0);
        let ticks_unix = ticks_ua - i128::from(UA_DATETIME_UNIX_EPOCH);
        let nanos_unix = ticks_unix * i128::from(1000 / UA_DATETIME_USEC);

        time::OffsetDateTime::from_unix_timestamp_nanos(nanos_unix).ok()
    }
}

#[cfg(feature = "time")]
impl TryFrom<time::OffsetDateTime> for DateTime {
    // Explicit module path to avoid linter errors when feature is not enable by `#[cfg()]`.
    type Error = crate::Error;

    /// Creates [`DateTime`] from [`time::OffsetDateTime`].
    ///
    /// # Examples
    ///
    /// ```
    /// use open62541::ua;
    /// use time::macros::datetime;
    ///
    /// let dt: ua::DateTime = datetime!(2024-02-09 12:34:56 UTC).try_into().unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// The date/time must be valid and in range of the 64-bit representation of [`DateTime`]
    /// within the limits [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    fn try_from(from: time::OffsetDateTime) -> Result<Self, Self::Error> {
        use open62541_sys::{UA_DATETIME_UNIX_EPOCH, UA_DATETIME_USEC};

        // OPC UA encodes `DateTime` as Windows file time: a 64-bit value that represents the number
        // of 100-nanosecond intervals that have elapsed since 12:00 A.M. January 1, 1601 (UTC).
        let nanos_unix = from.unix_timestamp_nanos();
        let ticks_unix = nanos_unix / i128::from(1000 / UA_DATETIME_USEC);
        let ticks_ua = ticks_unix + i128::from(UA_DATETIME_UNIX_EPOCH);

        // Limit to min/max values.
        if ticks_ua < Self::MIN.0.into() {
            return Err(crate::Error::internal("DateTime exceeds minimum value"));
        }
        if ticks_ua > Self::MAX.0.into() {
            return Err(crate::Error::internal("DateTime exceeds maximum value"));
        }

        i64::try_from(ticks_ua)
            // Explicit module path to avoid linter errors when feature is not enable by `#[cfg()]`.
            .map_err(|_| crate::Error::internal("DateTime should be in range"))
            .map(Self)
    }
}

#[cfg(all(feature = "serde", feature = "time"))]
impl serde::Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_utc()
            .ok_or(serde::ser::Error::custom("DateTime should be in range"))
            .and_then(|dt| time::serde::rfc3339::serialize(&dt, serializer))
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "time")]
    #[test]
    fn from_offset_to_utc() {
        // A timestamp with 100-nanosecond precision
        let dt = time::macros::datetime!(2023-11-20 16:51:15.9876543 -2:00);
        assert_eq!(time::macros::offset!(-2:00), dt.offset());
        assert_ne!(time::macros::offset!(UTC), dt.offset());
        let dt_ua = super::DateTime::try_from(dt).unwrap();
        let dt_utc = dt_ua.to_utc().unwrap();
        // Equal to the original timestamp, but the offset is now UTC.
        assert_eq!(time::macros::offset!(UTC), dt_utc.offset());
        assert_ne!(dt.offset(), dt_utc.offset());
        assert_eq!(dt, dt_utc);
    }

    #[cfg(feature = "time")]
    #[test]
    fn min_max_utc() {
        let min_ua = super::DateTime::try_from(super::DateTime::MIN_UTC).unwrap();
        assert_eq!(min_ua, super::DateTime::MIN);
        assert!(super::DateTime::try_from(
            super::DateTime(super::DateTime::MIN.0 - 1)
                .to_utc()
                .unwrap()
        )
        .is_err());

        let max_ua = super::DateTime::try_from(super::DateTime::MAX_UTC).unwrap();
        assert_eq!(max_ua, super::DateTime::MAX);
        assert!(super::DateTime::try_from(
            super::DateTime(super::DateTime::MAX.0 + 1)
                .to_utc()
                .unwrap()
        )
        .is_err());
    }
}
