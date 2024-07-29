use crate::ua;

crate::data_type!(CreateMonitoredItemsResponse);

impl CreateMonitoredItemsResponse {
    #[must_use]
    pub fn monitored_item_ids(&self) -> Option<Vec<ua::MonitoredItemId>> {
        let results = ua::Array::<ua::MonitoredItemCreateResult>::from_raw_parts(
            self.0.resultsSize,
            self.0.results,
        )?;

        let monitored_item_ids: Vec<_> = results
            .as_slice()
            .iter()
            .map(ua::MonitoredItemCreateResult::monitored_item_id)
            .collect();

        Some(monitored_item_ids)
    }
}
