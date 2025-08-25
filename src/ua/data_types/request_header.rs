use std::time::Duration;

crate::data_type!(RequestHeader);

impl RequestHeader {
    /// Gets the timeout hint.
    #[must_use]
    pub const fn timeout_hint(&self) -> Duration {
        #[expect(
            clippy::as_conversions,
            reason = "infallible conversion from u32 to 64 in const fn"
        )]
        Duration::from_millis(self.0.timeoutHint as u64)
    }

    /// Sets a custom response timeout a request.
    ///
    /// Overrides the response timeout of the client.
    ///
    /// The minimum value is 1 ms. Values greater than zero but less than 1 ms are set to 1 ms.
    /// A value of [`Duration::ZERO`] disables the timeout.
    ///
    /// The maximum supported value is [`u32::MAX`] ms. Greater values are clamped to this limit.
    pub fn set_timeout_hint(&mut self, timeout_hint: Duration) {
        let mut millis = u32::try_from(timeout_hint.as_millis())
            .ok()
            // Clamp to maximum value.
            .unwrap_or(u32::MAX);
        if !timeout_hint.is_zero() && millis == 0 {
            // Minimum response timeout is 1 ms.
            millis = 1;
        }
        self.0.timeoutHint = millis;
    }
}

#[cfg(test)]
mod tests {
    use crate::DataType as _;

    use super::*;

    #[test]
    fn should_set_timeout_hint_to_zero() {
        let non_zero_duration = Duration::from_secs(1);
        let mut request_header = RequestHeader::init();
        request_header.set_timeout_hint(non_zero_duration);
        assert_eq!(request_header.timeout_hint(), non_zero_duration);
        request_header.set_timeout_hint(Duration::ZERO);
        assert_eq!(request_header.timeout_hint(), Duration::ZERO);
    }

    #[test]
    fn should_clamp_min_timeout_hint() {
        let min_duration = Duration::from_millis(1);
        let below_min_duration = min_duration.checked_div(2).unwrap();
        let mut request_header = RequestHeader::init();
        request_header.set_timeout_hint(below_min_duration);
        assert_eq!(request_header.timeout_hint(), min_duration);
    }

    #[test]
    fn should_clamp_max_timeout_hint() {
        let max_duration = Duration::from_millis(u32::MAX.into());
        let mut request_header = RequestHeader::init();
        request_header.set_timeout_hint(max_duration + Duration::from_secs(1));
        assert_eq!(request_header.timeout_hint(), max_duration);
    }
}
