use std::time::Duration;

use crate::{Error, Result, ua};

crate::data_type!(CreateSubscriptionResponse);
crate::member_accessors!(CreateSubscriptionResponse {
    #[skip(uses = Self::subscription_id)]
    subscriptionId: ua::SubscriptionId,
    revisedLifetimeCount: u32,
    revisedMaxKeepAliveCount: u32,
    #[skip(uses = Self::revised_publishing_interval)]
    revisedPublishingInterval: f64,
    responseHeader: &ua::ResponseHeader
});

impl CreateSubscriptionResponse {
    #[must_use]
    pub const fn subscription_id(&self) -> Option<ua::SubscriptionId> {
        if let Some(id) = ua::IntegerId::from_u32(self.0.subscriptionId) {
            Some(ua::SubscriptionId::new(id))
        } else {
            None
        }
    }

    /// Gets revised publishing interval.
    ///
    /// # Errors
    ///
    /// This fails when the returned value is negative.
    pub fn revised_publishing_interval(&self) -> Result<Duration> {
        Duration::try_from_secs_f64(self.0.revisedPublishingInterval / 1e3)
            .map_err(|_| Error::internal("invalid revised publishing interval"))
    }
}
