use std::time::Duration;

use crate::{ua, DataType as _, ServiceRequest};

crate::data_type!(ReadRequest);

impl ReadRequest {
    #[must_use]
    pub fn with_timestamps_to_return(
        mut self,
        timestamps_to_return: &ua::TimestampsToReturn,
    ) -> Self {
        timestamps_to_return.clone_into_raw(&mut self.0.timestampsToReturn);
        self
    }

    #[must_use]
    pub fn with_nodes_to_read(mut self, nodes_to_read: &[ua::ReadValueId]) -> Self {
        let array = ua::Array::from_slice(nodes_to_read);
        array.move_into_raw(&mut self.0.nodesToReadSize, &mut self.0.nodesToRead);
        self
    }

    /// Sets a custom response timeout for this request.
    ///
    /// Overrides the response timeout of the client.
    ///
    /// The minimum value is 1 ms. A value of [`Duration::ZERO`] disables the timeout (not recommended).
    #[must_use]
    pub fn with_timeout_hint(mut self, timeout_hint: Duration) -> Self {
        let mut millis = u32::try_from(timeout_hint.as_millis())
            .ok()
            .unwrap_or(u32::MAX);
        if !timeout_hint.is_zero() && millis == 0 {
            // Minimum response timeout is 1 ms.
            millis = 1;
        }
        self.0.requestHeader.timeoutHint = millis;
        self
    }
}

impl ServiceRequest for ReadRequest {
    type Response = ua::ReadResponse;
}
