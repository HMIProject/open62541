use std::time::Duration;

use crate::{DataType as _, MonitoringFilter};

crate::data_type!(MonitoringParameters);

impl MonitoringParameters {
    /// Sets sampling interval.
    ///
    /// The value `Some(Duration::ZERO)` indicates that the server should use the fastest practical
    /// rate.
    ///
    /// The value `None` (-1) indicates that the default sampling interval defined by the publishing
    /// interval of the subscription is requested.
    #[must_use]
    pub fn with_sampling_interval(mut self, sampling_interval: Option<Duration>) -> Self {
        self.0.samplingInterval = if let Some(sampling_interval) = sampling_interval {
            sampling_interval.as_secs_f64() * 1e3
        } else {
            -1.0
        };
        self
    }

    /// Sets filter.
    #[must_use]
    pub fn with_filter(mut self, filter: &impl MonitoringFilter) -> Self {
        filter
            .to_extension_object()
            .move_into_raw(&mut self.0.filter);
        self
    }

    /// Sets requested size of the monitored item queue.
    #[must_use]
    pub const fn with_queue_size(mut self, queue_size: u32) -> Self {
        self.0.queueSize = queue_size;
        self
    }

    /// Sets discard policy.
    ///
    /// When `true`, the oldest notification in the queue is discarded and the new notification is
    /// added to the end of the queue. When `false`, the last notification added to the queue gets
    /// replaced with the new notification.
    #[must_use]
    pub const fn with_discard_oldest(mut self, discard_oldest: bool) -> Self {
        self.0.discardOldest = discard_oldest;
        self
    }
}
