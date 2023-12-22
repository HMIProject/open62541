use crate::ua;

crate::data_type!(DeleteMonitoredItemsRequest);

impl DeleteMonitoredItemsRequest {
    #[must_use]
    pub fn with_monitored_item_ids(mut self, monitored_item_ids: &[ua::MonitoredItemId]) -> Self {
        let array = ua::Array::from_iter(
            monitored_item_ids
                .iter()
                .map(|monitored_item_id| monitored_item_id.to_uint32()),
        );
        array.move_into_raw(
            &mut self.0.monitoredItemIdsSize,
            &mut self.0.monitoredItemIds,
        );
        self
    }
}
