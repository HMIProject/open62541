use crate::{ua, DataType as _, Error, Result, ServiceResponse};

crate::data_type!(BrowseNextResponse);

impl BrowseNextResponse {
    #[must_use]
    pub fn results(&self) -> Option<ua::Array<ua::BrowseResult>> {
        // TODO: Adjust signature to return non-owned value instead.
        ua::Array::from_raw_parts(self.0.resultsSize, self.0.results)
    }

    /// Evaluates the response as a [`Result`] containing multiple [`BrowseResult`](crate::BrowseResult)s.
    ///
    /// # Errors
    ///
    /// Fails only when the entire request fails or when the number of browse results
    /// differs from the given `expected_results_len`. When a continuation point is invalid
    /// an inner `Err` is returned.
    pub fn eval_many(&self, expected_results_len: usize) -> Result<Vec<crate::BrowseResult>> {
        let Some(results) = self.results() else {
            return Err(Error::internal("browse should return results"));
        };

        // The OPC UA specification state that the resulting list has the same number of elements as
        // the request list. If not, we would not be able to match elements in the two lists anyway.
        if results.len() != expected_results_len {
            return Err(Error::Internal("unexpected number of browse results"));
        }

        Ok(results.iter().map(ua::BrowseResult::eval).collect())
    }
}

impl ServiceResponse for BrowseNextResponse {
    type Request = ua::BrowseNextRequest;

    fn response_header(&self) -> &ua::ResponseHeader {
        ua::ResponseHeader::raw_ref(&self.0.responseHeader)
    }
}
