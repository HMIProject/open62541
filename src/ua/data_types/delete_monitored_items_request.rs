use crate::{UaIterExt as _, ua};

crate::data_type!(DeleteMonitoredItemsRequest);
crate::member_accessors!(DeleteMonitoredItemsRequest {
    #[skip(uses = Self::subscription_id)]
    subscriptionId: ua::SubscriptionId,
    monitoredItemIds: [ua::UInt32]
});

impl DeleteMonitoredItemsRequest {
    #[must_use]
    pub const fn with_subscription_id(mut self, subscription_id: ua::SubscriptionId) -> Self {
        self.0.subscriptionId = subscription_id.as_id().as_u32();
        self
    }

    #[must_use]
    pub fn with_monitored_item_ids(mut self, monitored_item_ids: &[ua::MonitoredItemId]) -> Self {
        monitored_item_ids
            .iter()
            .map(|monitored_item_id| monitored_item_id.as_id().to_uint32())
            .collect_ua_array()
            .move_into_raw(
                &mut self.0.monitoredItemIdsSize,
                &mut self.0.monitoredItemIds,
            );
        self
    }

    #[must_use]
    pub const fn subscription_id(&self) -> Option<ua::SubscriptionId> {
        if let Some(id) = ua::IntegerId::from_u32(self.0.subscriptionId) {
            Some(ua::SubscriptionId::new(id))
        } else {
            None
        }
    }
}
