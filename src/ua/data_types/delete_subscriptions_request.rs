use crate::ua;

crate::data_type!(DeleteSubscriptionsRequest);

impl DeleteSubscriptionsRequest {
    #[must_use]
    pub fn with_subscription_ids(mut self, subscription_ids: &[ua::SubscriptionId]) -> Self {
        let array = ua::Array::from_iter(
            subscription_ids
                .iter()
                .map(|subscription_id| subscription_id.as_id().to_uint32()),
        );
        array.move_into_raw(&mut self.0.subscriptionIdsSize, &mut self.0.subscriptionIds);
        self
    }
}
