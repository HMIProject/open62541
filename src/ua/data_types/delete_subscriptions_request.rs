use crate::{ua, SubscriptionId};

crate::data_type!(
    DeleteSubscriptionsRequest,
    UA_DeleteSubscriptionsRequest,
    UA_TYPES_DELETESUBSCRIPTIONSREQUEST
);

impl DeleteSubscriptionsRequest {
    #[must_use]
    pub fn with_subscription_ids(mut self, subscription_ids: &[SubscriptionId]) -> Self {
        let array = ua::Array::from_iter(subscription_ids.iter().map(|id| ua::Uint32::new(id.0)));

        // Make sure to clean up any previous value in target.
        let _unused = ua::Array::<ua::Uint32>::from_raw_parts(
            self.0.subscriptionIds,
            self.0.subscriptionIdsSize,
        );

        // Transfer ownership from `array` into `self`.
        let (size, ptr) = array.into_raw_parts();
        self.0.subscriptionIdsSize = size;
        self.0.subscriptionIds = ptr;

        self
    }
}
