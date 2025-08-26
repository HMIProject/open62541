use crate::{ua, DataType as _, Error, Result, ServiceResponse};

crate::data_type!(CallResponse);

impl CallResponse {
    #[must_use]
    pub fn results(&self) -> Option<ua::Array<ua::CallMethodResult>> {
        // TODO: Adjust signature to return non-owned value instead.
        ua::Array::from_raw_parts(self.0.resultsSize, self.0.results)
    }

    /// Evaluates the response as a [`Result`].
    ///
    /// # Errors
    ///
    /// Fails when the object or method node does not exist, the method cannot be called, or
    /// the input arguments are unexpected.
    pub fn eval(&self, method_id: &ua::NodeId) -> Result<Vec<ua::Variant>> {
        let Some(results) = self.results() else {
            return Err(Error::internal("call should return results"));
        };

        if results.as_slice().len() != 1 {
            return Err(Error::internal("call should return a single result"));
        }
        #[expect(clippy::missing_panics_doc, reason = "Length has just been checked.")]
        let result = results.as_slice().first().expect("single result");

        Error::verify_good(&result.status_code())?;

        let output_arguments = if let Some(output_arguments) = result.output_arguments() {
            output_arguments.into_vec()
        } else {
            log::debug!("Calling {method_id} returned unset output arguments, assuming none exist");
            Vec::new()
        };

        Ok(output_arguments)
    }
}

impl ServiceResponse for CallResponse {
    type Request = ua::CallRequest;

    fn response_header(&self) -> &ua::ResponseHeader {
        ua::ResponseHeader::raw_ref(&self.0.responseHeader)
    }
}
