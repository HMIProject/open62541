use std::time::Duration;

use crate::{Error, Result, ua};

crate::data_type!(MonitoredItemCreateResult);
crate::member_accessors!(MonitoredItemCreateResult {
    #[from_inner]
    statusCode: ua::StatusCode,
    #[skip(uses = Self::monitored_item_id)]
    monitoredItemId: ua::MonitoredItemId,
    revisedQueueSize: u32,
    #[skip(uses = Self::revised_sampling_interval)]
    revisedSamplingInterval: f64,
    filterResult: &ua::ExtensionObject
});

impl MonitoredItemCreateResult {
    #[must_use]
    pub(crate) const fn monitored_item_id(&self) -> Option<ua::MonitoredItemId> {
        if let Some(id) = ua::IntegerId::from_u32(self.0.monitoredItemId) {
            Some(ua::MonitoredItemId::new(id))
        } else {
            None
        }
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
}
