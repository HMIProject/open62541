use crate::ua;

crate::data_type!(
    MonitoredItemCreateResult,
    UA_MonitoredItemCreateResult,
    UA_TYPES_MONITOREDITEMCREATERESULT
);

impl MonitoredItemCreateResult {
    #[must_use]
    pub const fn monitored_item_id(&self) -> ua::MonitoredItemId {
        ua::MonitoredItemId::new(self.0.monitoredItemId)
    }
}
