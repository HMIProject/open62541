use std::{num::NonZeroU32, time::Duration};

use open62541_sys::UA_CreateSubscriptionRequest_default;

crate::data_type!(CreateSubscriptionRequest);

impl CreateSubscriptionRequest {
    /// Sets requested publishing interval.
    ///
    /// The value `None` indicates that the server shall revise with the fastest supported
    /// publishing interval
    #[must_use]
    pub fn with_requested_publishing_interval(
        mut self,
        requested_publishing_interval: Option<Duration>,
    ) -> Self {
        // The special value `0` (or negative) indicates that the server shall revise with the
        // fastest supported publishing interval.
        self.0.requestedPublishingInterval = requested_publishing_interval
            .map_or(0.0, |requested_publishing_interval| {
                requested_publishing_interval.as_secs_f64() * 1e3
            });
        self
    }

    /// Sets requested lifetime count.
    #[must_use]
    pub const fn with_requested_lifetime_count(mut self, requested_lifetime_count: u32) -> Self {
        self.0.requestedLifetimeCount = requested_lifetime_count;
        self
    }

    /// Sets requested maximum keep-alive count.
    ///
    /// The value `None` indicates that the server shall revise with the smallest supported
    /// keep-alive count.
    #[must_use]
    pub fn with_requested_max_keep_alive_count(
        mut self,
        requested_max_keep_alive_count: Option<NonZeroU32>,
    ) -> Self {
        // The special value `0` indicates that the server shall revise with the smallest supported
        // keep-alive count.
        self.0.requestedMaxKeepAliveCount =
            requested_max_keep_alive_count.map_or(0, NonZeroU32::get);
        self
    }

    /// Sets maximum number of notifications that the client wishes to receive in a single publish
    /// response.
    ///
    /// The value `None` indicates that there is no limit.
    #[must_use]
    pub fn with_max_notifications_per_publish(
        mut self,
        max_notifications_per_publish: Option<NonZeroU32>,
    ) -> Self {
        // The special value `0` indicates that there is no limit.
        self.0.maxNotificationsPerPublish =
            max_notifications_per_publish.map_or(0, NonZeroU32::get);
        self
    }

    /// Enables or disables publishing.
    #[must_use]
    pub const fn with_publishing_enabled(mut self, publishing_enabled: bool) -> Self {
        self.0.publishingEnabled = publishing_enabled;
        self
    }

    /// Sets relative priority of the subscription.
    #[must_use]
    pub const fn with_priority(mut self, priority: u8) -> Self {
        self.0.priority = priority;
        self
    }
}

impl Default for CreateSubscriptionRequest {
    fn default() -> Self {
        let inner = unsafe { UA_CreateSubscriptionRequest_default() };
        Self(inner)
    }
}
