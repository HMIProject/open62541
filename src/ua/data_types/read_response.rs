use std::slice;

use crate::ua;

crate::data_type!(ReadResponse, UA_ReadResponse, UA_TYPES_READRESPONSE);

impl ReadResponse {
    #[must_use]
    pub fn results(&self) -> Vec<ua::DataValue> {
        let results = unsafe { slice::from_raw_parts(self.0.results, self.0.resultsSize) };
        results.iter().map(ua::DataValue::new_from).collect()
    }
}
