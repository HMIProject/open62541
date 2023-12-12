use crate::SubscriptionId;

crate::data_type!(
    CreateSubscriptionResponse,
    UA_CreateSubscriptionResponse,
    UA_TYPES_CREATESUBSCRIPTIONRESPONSE
);

impl CreateSubscriptionResponse {
    #[must_use]
    pub const fn subscription_id(&self) -> SubscriptionId {
        SubscriptionId(self.0.subscriptionId)
    }
}
