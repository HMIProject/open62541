crate::data_type!(DateTime);

#[cfg(feature = "time")]
impl DateTime {
    pub fn new(dt: time::OffsetDateTime) -> Option<Self> {
        use open62541_sys::{UA_DATETIME_UNIX_EPOCH, UA_DATETIME_USEC};

        // OPC UA encodes `DateTime` as Windows file time: a 64-bit value that represents the number
        // of 100-nanosecond intervals that have elapsed since 12:00 A.M. January 1, 1601 (UTC).
        let nanos_unix = dt.unix_timestamp_nanos();
        let ticks_unix = nanos_unix / i128::from(1000 / UA_DATETIME_USEC);
        let ticks_ua = ticks_unix + i128::from(UA_DATETIME_UNIX_EPOCH);
        i64::try_from(ticks_ua).ok().map(Self)
    }

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

#[cfg(test)]
mod tests {
    #[cfg(feature = "time")]
    #[test]
    fn from_offset_to_utc() {
        // A timestamp with 100-nanosecond precision
        let dt = time::macros::datetime!(2023-11-20 16:51:15.9876543 -2:00);
        assert_eq!(time::macros::offset!(-2:00), dt.offset());
        assert_ne!(time::macros::offset!(UTC), dt.offset());
        let dt_ua = crate::ua::DateTime::new(dt).unwrap();
        let dt_utc = dt_ua.to_utc().unwrap();
        // Equal to the original timestamp, but the offset is now UTC.
        assert_eq!(time::macros::offset!(UTC), dt_utc.offset());
        assert_ne!(dt.offset(), dt_utc.offset());
        assert_eq!(dt, dt_utc);
    }
}
