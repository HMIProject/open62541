use crate::{ua, BrowseResult, DataType as _, Error, Result, ServiceResponse};

crate::data_type!(BrowseNextResponse);

impl BrowseNextResponse {
    #[must_use]
    pub fn results(&self) -> Option<ua::Array<ua::BrowseResult>> {
        // TODO: Adjust signature to return non-owned value instead.
        ua::Array::from_raw_parts(self.0.resultsSize, self.0.results)
    }

    pub fn eval(&self, continuation_points: &[ua::ContinuationPoint]) -> Result<Vec<BrowseResult>> {
        let Some(results) = self.results() else {
            return Err(Error::internal("browse should return results"));
        };

        // The OPC UA specification state that the resulting list has the same number of elements as
        // the request list. If not, we would not be able to match elements in the two lists anyway.
        if results.len() != continuation_points.len() {
            return Err(Error::Internal("unexpected number of browse results"));
        }

        let results: Vec<_> = results.iter().map(|result| result.eval(None)).collect();

        Ok(results)
    }
}

impl ServiceResponse for BrowseNextResponse {
    type Request = ua::BrowseNextRequest;

    fn response_header(&self) -> &ua::ResponseHeader {
        ua::ResponseHeader::raw_ref(&self.0.responseHeader)
    }
}
