use crate::{ua, ServiceResponse};

crate::data_type!(WriteResponse, UA_WriteResponse, UA_TYPES_WRITERESPONSE);

impl WriteResponse {
    #[must_use]
    pub fn results(&self) -> Option<Vec<ua::StatusCode>> {
        // TODO: Adjust signature to return non-owned value instead.
        let array: ua::Array<ua::Uint32> =
            ua::Array::from_raw_parts(self.0.results, self.0.resultsSize)?;
        // TODO: Simplify this. Think about what should be in `ua` and what should not.
        Some(
            array
                .as_slice()
                .iter()
                .map(|status_code| ua::StatusCode::new(status_code.clone().into_inner()))
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
