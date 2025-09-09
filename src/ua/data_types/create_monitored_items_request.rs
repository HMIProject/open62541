#![cfg_attr(
    not(any(feature = "tokio", feature = "experimental-monitored-item-callback")),
    expect(
        dead_code,
        reason = "Some methods are only used when at least one of the features is enabled."
    )
)]

use crate::ua;

crate::data_type!(CreateMonitoredItemsRequest);

impl CreateMonitoredItemsRequest {
    #[must_use]
    pub const fn with_subscription_id(mut self, subscription_id: ua::SubscriptionId) -> Self {
        self.0.subscriptionId = subscription_id.as_u32();
        self
    }

    #[must_use]
    pub fn with_items_to_create(
        mut self,
        items_to_create: &[ua::MonitoredItemCreateRequest],
    ) -> Self {
        let array = ua::Array::from_slice(items_to_create);
        array.move_into_raw(&mut self.0.itemsToCreateSize, &mut self.0.itemsToCreate);
        self
    }

    #[must_use]
    pub(crate) fn items_to_create(&self) -> Option<&[ua::MonitoredItemCreateRequest]> {
        unsafe { ua::Array::slice_from_raw_parts(self.0.itemsToCreateSize, self.0.itemsToCreate) }
    }
}
