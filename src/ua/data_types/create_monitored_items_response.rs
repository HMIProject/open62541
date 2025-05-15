use crate::ua;

crate::data_type!(CreateMonitoredItemsResponse);

impl CreateMonitoredItemsResponse {
    #[allow(dead_code)] // This is unused for now.
    pub(crate) fn results(&self) -> Option<&[ua::MonitoredItemCreateResult]> {
        unsafe { ua::Array::slice_from_raw_parts(self.0.resultsSize, self.0.results) }
    }

    #[allow(dead_code, reason = "--no-default-features")]
    pub(crate) fn into_results(mut self) -> Option<ua::Array<ua::MonitoredItemCreateResult>> {
        unsafe { ua::Array::move_from_raw_parts(&mut self.0.resultsSize, &mut self.0.results) }
    }
}
