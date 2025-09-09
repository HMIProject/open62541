#![cfg_attr(
    not(any(feature = "tokio", feature = "experimental-monitored-item-callback")),
    expect(
        dead_code,
        reason = "Some methods are only used when at least one of the features is enabled."
    )
)]

use crate::ua;

crate::data_type!(CreateMonitoredItemsResponse);

impl CreateMonitoredItemsResponse {
    #[expect(dead_code, reason = "unused for now")]
    pub(crate) fn results(&self) -> Option<&[ua::MonitoredItemCreateResult]> {
        unsafe { ua::Array::slice_from_raw_parts(self.0.resultsSize, self.0.results) }
    }

    pub(crate) fn into_results(mut self) -> Option<ua::Array<ua::MonitoredItemCreateResult>> {
        unsafe { ua::Array::move_from_raw_parts(&mut self.0.resultsSize, &mut self.0.results) }
    }
}
