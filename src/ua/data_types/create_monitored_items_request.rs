use open62541_sys::UA_TimestampsToReturn;

use crate::ua;

crate::data_type!(CreateMonitoredItemsRequest);
crate::member_accessors!(CreateMonitoredItemsRequest {
    itemsToCreate: [ua::MonitoredItemCreateRequest],
    #[enum(UA_TimestampsToReturn)]
    timestampsToReturn: ua::TimestampsToReturn,
    #[skip(uses = Self::subscription_id)]
    subscriptionId: ua::SubscriptionId
});

impl CreateMonitoredItemsRequest {
    #[must_use]
    pub const fn with_subscription_id(mut self, subscription_id: ua::SubscriptionId) -> Self {
        self.0.subscriptionId = subscription_id.as_id().as_u32();
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
    pub const fn subscription_id(&self) -> Option<ua::SubscriptionId> {
        if let Some(id) = ua::IntegerId::from_u32(self.0.subscriptionId) {
            Some(ua::SubscriptionId(id))
        } else {
            None
        }
    }
}
