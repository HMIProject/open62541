use crate::{ua, DataType as _, DataValue, Error, Result, ServiceResponse};

crate::data_type!(ReadResponse);

impl ReadResponse {
    #[must_use]
    pub fn results(&self) -> Option<ua::Array<ua::DataValue>> {
        // TODO: Adjust signature to return non-owned value instead.
        ua::Array::from_raw_parts(self.0.resultsSize, self.0.results)
    }

    pub fn eval(&self, expected_len: usize) -> Result<Vec<DataValue<ua::Variant>>> {
        let Some(mut results) = self.results() else {
            return Err(Error::internal("read should return results"));
        };

        let results: Vec<DataValue<ua::Variant>> =
            results.drain_all().map(ua::DataValue::cast).collect();

        // The OPC UA specification state that the resulting list has the same number of elements as
        // the request list. If not, we would not be able to match elements in the two lists anyway.
        if results.len() != expected_len {
            return Err(Error::internal("unexpected number of read results"));
        }

        Ok(results)
    }
}

impl ServiceResponse for ReadResponse {
    type Request = ua::ReadRequest;

    fn response_header(&self) -> &ua::ResponseHeader {
        ua::ResponseHeader::raw_ref(&self.0.responseHeader)
    }
}
