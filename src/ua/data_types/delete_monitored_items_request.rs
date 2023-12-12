use crate::ua;

crate::data_type!(
    DeleteMonitoredItemsRequest,
    UA_DeleteMonitoredItemsRequest,
    UA_TYPES_DELETEMONITOREDITEMSREQUEST
);

impl DeleteMonitoredItemsRequest {
    #[must_use]
    pub fn with_monitored_item_ids(mut self, monitored_item_ids: &[ua::MonitoredItemId]) -> Self {
        let array = ua::Array::from_iter(
            monitored_item_ids
                .iter()
                .map(|id| ua::Uint32::new(id.into_inner())),
        );

        // Make sure to clean up any previous value in target.
        let _unused = ua::Array::<ua::Uint32>::from_raw_parts(
            self.0.monitoredItemIds,
            self.0.monitoredItemIdsSize,
        );

        // Transfer ownership from `array` into `self`.
        let (size, ptr) = array.into_raw_parts();
        self.0.monitoredItemIdsSize = size;
        self.0.monitoredItemIds = ptr;

        self
    }
}
