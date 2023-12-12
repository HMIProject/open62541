use crate::ua;

crate::data_type!(
    CreateSubscriptionResponse,
    UA_CreateSubscriptionResponse,
    UA_TYPES_CREATESUBSCRIPTIONRESPONSE
);

impl CreateSubscriptionResponse {
    #[must_use]
    pub const fn subscription_id(&self) -> ua::SubscriptionId {
        ua::SubscriptionId::new(self.0.subscriptionId)
    }
}
