use crate::{ua, DataType as _, DataValue, Error, Result, ServiceResponse};

crate::data_type!(ReadResponse);

impl ReadResponse {
    #[must_use]
    pub fn results(&self) -> Option<ua::Array<ua::DataValue>> {
        // TODO: Adjust signature to return non-owned value instead.
        ua::Array::from_raw_parts(self.0.resultsSize, self.0.results)
    }

    /// Evaluates the response as a [`Result`] with `expected_results_len` individual read results.
    ///
    /// # Errors
    ///
    /// This fails when a node does not exist or one of the given attributes cannot be read,
    /// the server returns a corresponding [`DataValue`] with the appropriate [`status()`]
    /// and with [`value()`] unset.
    ///
    /// [`status()`]: DataValue::status
    /// [`value()`]: DataValue::value
    pub fn eval_many(&self, expected_results_len: usize) -> Result<Vec<DataValue<ua::Variant>>> {
        let Some(mut results) = self.results() else {
            return Err(Error::internal("read should return results"));
        };

        // The OPC UA specification state that the resulting list has the same number of elements as
        // the request list. If not, we would not be able to match elements in the two lists anyway.
        if results.len() != expected_results_len {
            return Err(Error::internal("unexpected number of read results"));
        }

        Ok(results.drain_all().map(ua::DataValue::cast).collect())
    }
}

impl ServiceResponse for ReadResponse {
    type Request = ua::ReadRequest;

    fn response_header(&self) -> &ua::ResponseHeader {
        ua::ResponseHeader::raw_ref(&self.0.responseHeader)
    }
}
