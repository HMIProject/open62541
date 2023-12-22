use crate::ua;

crate::data_type!(CreateSubscriptionResponse);

impl CreateSubscriptionResponse {
    #[must_use]
    pub const fn subscription_id(&self) -> ua::SubscriptionId {
        ua::SubscriptionId::new(self.0.subscriptionId)
    }
}
