use std::time::Duration;

use crate::{ua, Error, Result};

crate::data_type!(MonitoredItemCreateResult);

impl MonitoredItemCreateResult {
    #[cfg_attr(not(feature = "tokio"), expect(dead_code, reason = "unused"))]
    #[must_use]
    pub(crate) const fn status_code(&self) -> ua::StatusCode {
        ua::StatusCode::new(self.0.statusCode)
    }

    #[cfg_attr(not(feature = "tokio"), expect(dead_code, reason = "unused"))]
    #[must_use]
    pub(crate) const fn monitored_item_id(&self) -> ua::MonitoredItemId {
        ua::MonitoredItemId::new(self.0.monitoredItemId)
    }

    /// Gets revised sampling interval.
    ///
    /// # Errors
    ///
    /// This fails when the returned value is negative.
    pub fn revised_sampling_interval(&self) -> Result<Duration> {
        Duration::try_from_secs_f64(self.0.revisedSamplingInterval / 1e3)
            .map_err(|_| Error::internal("invalid revised sampling interval"))
    }

    /// Gets revised queue size.
    #[must_use]
    pub const fn revised_queue_size(&self) -> u32 {
        self.0.revisedQueueSize
    }
}
