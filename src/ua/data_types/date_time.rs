crate::data_type!(DateTime, UA_DateTime, UA_TYPES_DATETIME);

impl DateTime {
    #[cfg(feature = "time")]
    #[must_use]
    pub fn as_datetime(&self) -> Option<time::OffsetDateTime> {
        use open62541_sys::{UA_DATETIME_UNIX_EPOCH, UA_DATETIME_USEC};

        // OPC UA encodes `DateTime` as Windows file time: a 64-bit value that represents the number
        // of 100-nanosecond intervals that have elapsed since 12:00 A.M. January 1, 1601 (UTC).
        let nsec = (i128::from(self.0) - i128::from(UA_DATETIME_UNIX_EPOCH))
            * i128::from(1000 / UA_DATETIME_USEC);
        time::OffsetDateTime::from_unix_timestamp_nanos(nsec).ok()
    }
}
