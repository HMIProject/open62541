use std::time::Duration;

crate::data_type!(RequestHeader);

impl RequestHeader {
    /// Sets a custom response timeout a request.
    ///
    /// Overrides the response timeout of the client.
    ///
    /// The minimum value is 1 ms. A value of [`Duration::ZERO`] disables the timeout (not recommended).
    pub fn set_timeout_hint(&mut self, timeout_hint: Duration) {
        let mut millis = u32::try_from(timeout_hint.as_millis())
            .ok()
            .unwrap_or(u32::MAX);
        if !timeout_hint.is_zero() && millis == 0 {
            // Minimum response timeout is 1 ms.
            millis = 1;
        }
        self.0.timeoutHint = millis;
    }
}
