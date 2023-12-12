use crate::ua;

crate::data_type!(
    CreateMonitoredItemsResponse,
    UA_CreateMonitoredItemsResponse,
    UA_TYPES_CREATEMONITOREDITEMSRESPONSE
);

impl CreateMonitoredItemsResponse {
    #[must_use]
    pub fn monitored_item_ids(&self) -> Option<Vec<ua::MonitoredItemId>> {
        let results = ua::Array::<ua::MonitoredItemCreateResult>::from_raw_parts(
            self.0.results,
            self.0.resultsSize,
        )?;

        let monitored_item_ids: Vec<_> = results
            .as_slice()
            .iter()
            .map(ua::MonitoredItemCreateResult::monitored_item_id)
            .collect();

        Some(monitored_item_ids)
    }
}
