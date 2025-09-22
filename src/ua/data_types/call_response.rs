use crate::{DataType as _, ServiceResponse, ua};

crate::data_type!(CallResponse);

impl CallResponse {
    #[must_use]
    pub fn results(&self) -> Option<ua::Array<ua::CallMethodResult>> {
        // TODO: Adjust signature to return non-owned value instead.
        ua::Array::from_raw_parts(self.0.resultsSize, self.0.results)
    }
}

impl ServiceResponse for CallResponse {
    type Request = ua::CallRequest;

    fn response_header(&self) -> &ua::ResponseHeader {
        ua::ResponseHeader::raw_ref(&self.0.responseHeader)
    }
}
