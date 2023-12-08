use crate::MonitoredItemId;

crate::data_type!(
    MonitoredItemCreateResult,
    UA_MonitoredItemCreateResult,
    UA_TYPES_MONITOREDITEMCREATERESULT
);

impl MonitoredItemCreateResult {
    #[must_use]
    pub fn monitored_item_id(&self) -> MonitoredItemId {
        MonitoredItemId(self.0.monitoredItemId)
    }
}
