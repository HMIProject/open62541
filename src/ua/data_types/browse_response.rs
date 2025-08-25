use crate::{ua, DataType as _, Error, Result, ServiceResponse};

crate::data_type!(BrowseResponse);

impl BrowseResponse {
    #[must_use]
    pub fn results(&self) -> Option<ua::Array<ua::BrowseResult>> {
        // TODO: Adjust signature to return non-owned value instead.
        ua::Array::from_raw_parts(self.0.resultsSize, self.0.results)
    }

    /// Evaluates the response and converts it into the corresponding result type.
    pub fn eval(&self, browse_description: &ua::BrowseDescription) -> crate::BrowseResult {
        let Some(results) = self.results() else {
            return Err(Error::internal("browse should return results"));
        };

        let Some(result) = results.as_slice().first() else {
            return Err(Error::internal("browse should return a result"));
        };

        result.eval(Some(browse_description.node_id()))
    }

    /// Evaluates the response and converts it into the corresponding result type.
    pub fn eval_many(
        &self,
        browse_descriptions: &[ua::BrowseDescription],
    ) -> Result<Vec<crate::BrowseResult>> {
        let Some(results) = self.results() else {
            return Err(Error::internal("browse should return results"));
        };

        // The OPC UA specification state that the resulting list has the same number of elements as
        // the request list. If not, we would not be able to match elements in the two lists anyway.
        if results.len() != browse_descriptions.len() {
            return Err(Error::internal("unexpected number of browse results"));
        }

        let results: Vec<_> = results
            .iter()
            .zip(browse_descriptions)
            .map(|(result, browse_description)| result.eval(Some(browse_description.node_id())))
            .collect();

        Ok(results)
    }
}

impl ServiceResponse for BrowseResponse {
    type Request = ua::BrowseRequest;

    fn response_header(&self) -> &ua::ResponseHeader {
        ua::ResponseHeader::raw_ref(&self.0.responseHeader)
    }
}
