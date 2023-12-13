use crate::ua;

crate::data_type!(
    CreateMonitoredItemsRequest,
    UA_CreateMonitoredItemsRequest,
    UA_TYPES_CREATEMONITOREDITEMSREQUEST
);

impl CreateMonitoredItemsRequest {
    #[must_use]
    pub fn with_subscription_id(mut self, subscription_id: &ua::SubscriptionId) -> Self {
        self.0.subscriptionId = subscription_id.into_inner();
        self
    }

    #[must_use]
    pub fn with_items_to_create(
        mut self,
        items_to_create: &[ua::MonitoredItemCreateRequest],
    ) -> Self {
        let array = ua::Array::from_slice(items_to_create);

        // Make sure to clean up any previous value in target.
        let _unused = ua::Array::<ua::MonitoredItemCreateRequest>::from_raw_parts(
            self.0.itemsToCreate,
            self.0.itemsToCreateSize,
        );

        // Transfer ownership from `array` into `self`.
        let (size, ptr) = array.into_raw_parts();
        self.0.itemsToCreateSize = size;
        self.0.itemsToCreate = ptr;

        self
    }
}
