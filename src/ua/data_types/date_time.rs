use open62541_sys::{UA_DATETIME_UNIX_EPOCH, UA_DATETIME_USEC};

use crate::Error;

crate::data_type!(DateTime);

impl DateTime {
    /// Creates [`DateTime`] from a UNIX timestamp with nanosecond precision.
    ///
    /// /// # Examples
    ///
    /// ```
    /// use open62541::ua;
    ///
    /// // Unix timestamp (1707482096 seconds) corresponding to 9th February 2024, 12:34:56 UTC.
    /// let dt = ua::DateTime::try_from_unix_timestamp_nanos(1_707_482_096_000_000_000).unwrap();
    ///
    /// assert_eq!(format!("{dt:?}"), "\"2024-02-09T12:34:56Z\"");
    /// ```
    ///
    /// # Errors
    ///
    /// The UNIX timestamp must be valid and in range of the 64-bit representation of [`DateTime`].
    pub fn try_from_unix_timestamp_nanos(unix_timestamp_nanos: i128) -> Result<Self, Error> {
        // OPC UA encodes `DateTime` as Windows file time: a 64-bit value that represents the number
        // of 100-nanosecond intervals that have elapsed since 12:00 A.M. January 1, 1601 (UTC).
        let unix_ticks = unix_timestamp_nanos / i128::from(1000 / UA_DATETIME_USEC);
        let ua_ticks = unix_ticks + i128::from(UA_DATETIME_UNIX_EPOCH);

        i64::try_from(ua_ticks)
            .map_err(|_| Error::internal("DateTime should be in range"))
            .map(Self)
    }

    /// Returns the UNIX timestamp with nanosecond precision.
    #[must_use]
    pub fn as_unix_timestamp_nanos(&self) -> i128 {
        // OPC UA encodes `DateTime` as Windows file time: a 64-bit value that represents the number
        // of 100-nanosecond intervals that have elapsed since 12:00 A.M. January 1, 1601 (UTC).
        let ua_ticks = i128::from(self.0);
        let unix_ticks = ua_ticks - i128::from(UA_DATETIME_UNIX_EPOCH);

        unix_ticks * i128::from(1000 / UA_DATETIME_USEC)
    }
}

#[cfg(feature = "time")]
impl DateTime {
    // TODO (breaking change): Return `time::UtcDateTime` instead of `time::OffsetDateTime`.
    #[must_use]
    pub fn to_utc(&self) -> Option<time::OffsetDateTime> {
        time::OffsetDateTime::from_unix_timestamp_nanos(self.as_unix_timestamp_nanos()).ok()
    }
}

// TODO (breaking change): Upgrade `time` (0.3.38), add conversion from `time::UtcDateTime`.
#[cfg(feature = "time")]
impl TryFrom<time::OffsetDateTime> for DateTime {
    type Error = Error;

    /// Creates [`DateTime`] from [`time::OffsetDateTime`].
    ///
    /// # Examples
    ///
    /// ```
    /// use open62541::ua;
    /// use time::macros::datetime;
    ///
    /// let dt: ua::DateTime = datetime!(2024-02-09 12:34:56 UTC).try_into().unwrap();
    ///
    /// assert_eq!(format!("{dt:?}"), "\"2024-02-09T12:34:56Z\"");
    /// ```
    ///
    /// # Errors
    ///
    /// The date/time must be valid and in range of the 64-bit representation of [`DateTime`].
    fn try_from(from: time::OffsetDateTime) -> Result<Self, Self::Error> {
        Self::try_from_unix_timestamp_nanos(from.unix_timestamp_nanos())
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
        let dt_ua = crate::ua::DateTime::try_from(dt).unwrap();
        let dt_utc = dt_ua.to_utc().unwrap();
        // Equal to the original timestamp, but the offset is now UTC.
        assert_eq!(time::macros::offset!(UTC), dt_utc.offset());
        assert_ne!(dt.offset(), dt_utc.offset());
        assert_eq!(dt, dt_utc);
    }
}
