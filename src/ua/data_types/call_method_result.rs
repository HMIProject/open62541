use crate::ua;

crate::data_type!(CallMethodResult);

impl CallMethodResult {
    #[must_use]
    pub const fn status_code(&self) -> ua::StatusCode {
        ua::StatusCode::new(self.0.statusCode)
    }

    #[must_use]
    pub fn input_argument_results(&self) -> Option<ua::Array<ua::StatusCode>> {
        // TODO: Adjust signature to return non-owned value instead.
        ua::Array::from_raw_parts(self.0.inputArgumentResults, self.0.inputArgumentResultsSize)
    }

    #[must_use]
    pub fn output_arguments(&self) -> Option<ua::Array<ua::Variant>> {
        // TODO: Adjust signature to return non-owned value instead.
        ua::Array::from_raw_parts(self.0.outputArguments, self.0.outputArgumentsSize)
    }
}
