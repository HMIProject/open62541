crate::data_type!(DateTime);

impl DateTime {
    #[cfg(feature = "time")]
    #[must_use]
    pub fn to_utc(&self) -> Option<time::OffsetDateTime> {
        use open62541_sys::{UA_DATETIME_UNIX_EPOCH, UA_DATETIME_USEC};

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
    /// The date/time must be valid and in range of the 64-bit representation of [`DateTime`].
    fn try_from(from: time::OffsetDateTime) -> Result<Self, Self::Error> {
        use open62541_sys::{UA_DATETIME_UNIX_EPOCH, UA_DATETIME_USEC};

        // OPC UA encodes `DateTime` as Windows file time: a 64-bit value that represents the number
        // of 100-nanosecond intervals that have elapsed since 12:00 A.M. January 1, 1601 (UTC).
        let nanos_unix = from.unix_timestamp_nanos();
        let ticks_unix = nanos_unix / i128::from(1000 / UA_DATETIME_USEC);
        let ticks_ua = ticks_unix + i128::from(UA_DATETIME_UNIX_EPOCH);

        i64::try_from(ticks_ua)
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
        let dt_ua = crate::ua::DateTime::try_from(dt).unwrap();
        let dt_utc = dt_ua.to_utc().unwrap();
        // Equal to the original timestamp, but the offset is now UTC.
        assert_eq!(time::macros::offset!(UTC), dt_utc.offset());
        assert_ne!(dt.offset(), dt_utc.offset());
        assert_eq!(dt, dt_utc);
    }
}
