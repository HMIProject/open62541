use crate::ua;

crate::data_type!(ReadResponse, UA_ReadResponse, UA_TYPES_READRESPONSE);

impl ReadResponse {
    #[must_use]
    pub fn results(&self) -> Option<ua::Array<ua::DataValue>> {
        // TODO: Adjust signature to return non-owned value instead.
        ua::Array::from_raw_parts(self.0.results, self.0.resultsSize)
    }
}
