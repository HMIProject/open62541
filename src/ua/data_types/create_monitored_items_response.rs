use crate::{ua, MonitoredItemId};

crate::data_type!(
    CreateMonitoredItemsResponse,
    UA_CreateMonitoredItemsResponse,
    UA_TYPES_CREATEMONITOREDITEMSRESPONSE
);

impl CreateMonitoredItemsResponse {
    pub fn monitored_item_ids(&self) -> Option<Vec<MonitoredItemId>> {
        let results = ua::Array::<ua::MonitoredItemCreateResult>::from_raw_parts(
            self.0.results,
            self.0.resultsSize,
        )?;

        let monitored_item_ids: Vec<_> = results
            .as_slice()
            .iter()
            .map(|result| result.monitored_item_id())
            .collect();

        Some(monitored_item_ids)
    }
}
