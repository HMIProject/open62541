use crate::{ua, DataType as _, ServiceResponse};

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
}

impl ServiceResponse for WriteResponse {
    type Request = ua::WriteRequest;

    fn service_result(&self) -> ua::StatusCode {
        ua::StatusCode::new(self.0.responseHeader.serviceResult)
    }
}
