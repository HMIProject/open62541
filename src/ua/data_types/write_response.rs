use crate::{ua, DataType as _, Error, Result, ServiceResponse};

crate::data_type!(WriteResponse);

impl WriteResponse {
    #[must_use]
    pub fn results(&self) -> Option<Vec<ua::StatusCode>> {
        // TODO: Adjust signature to return non-owned value instead.
        let array: ua::Array<ua::UInt32> =
            ua::Array::from_raw_parts(self.0.resultsSize, self.0.results)?;
        // TODO: Simplify this. Think about what should be in `ua` and what should not.
        Some(
            array
                .as_slice()
                .iter()
                .map(|status_code| ua::StatusCode::new(status_code.clone().into_raw()))
                .collect(),
        )
    }

    /// Evaluates the response as a [`Result`].
    ///
    /// # Errors
    ///
    /// This fails when the node does not exist or its value attribute cannot be written.
    pub fn eval(&self) -> Result<()> {
        let Some(results) = self.results() else {
            return Err(Error::internal("write should return results"));
        };

        if results.as_slice().len() != 1 {
            return Err(Error::internal("write should return a single result"));
        }
        #[expect(clippy::missing_panics_doc, reason = "Length has just been checked.")]
        let result = results.as_slice().first().expect("single result");

        Error::verify_good(result)?;

        Ok(())
    }
}

impl ServiceResponse for WriteResponse {
    type Request = ua::WriteRequest;

    fn response_header(&self) -> &ua::ResponseHeader {
        ua::ResponseHeader::raw_ref(&self.0.responseHeader)
    }
}
