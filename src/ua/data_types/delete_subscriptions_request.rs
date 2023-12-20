use crate::ua;

crate::data_type!(
    DeleteSubscriptionsRequest,
    UA_DeleteSubscriptionsRequest,
    UA_TYPES_DELETESUBSCRIPTIONSREQUEST
);

impl DeleteSubscriptionsRequest {
    #[must_use]
    pub fn with_subscription_ids(mut self, subscription_ids: &[ua::SubscriptionId]) -> Self {
        let array = ua::Array::from_iter(
            subscription_ids
                .iter()
                .map(|id| ua::UInt32::new(id.into_inner())),
        );
        array.move_into(&mut self.0.subscriptionIdsSize, &mut self.0.subscriptionIds);
        self
    }
}
