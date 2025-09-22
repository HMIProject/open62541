use crate::{DataType as _, ServiceRequest, ua};

crate::data_type!(CallRequest);

impl CallRequest {
    #[must_use]
    pub fn with_methods_to_call(mut self, methods_to_call: &[ua::CallMethodRequest]) -> Self {
        let array = ua::Array::from_slice(methods_to_call);
        array.move_into_raw(&mut self.0.methodsToCallSize, &mut self.0.methodsToCall);
        self
    }
}

impl ServiceRequest for CallRequest {
    type Response = ua::CallResponse;

    fn request_header(&self) -> &ua::RequestHeader {
        ua::RequestHeader::raw_ref(&self.0.requestHeader)
    }

    fn request_header_mut(&mut self) -> &mut ua::RequestHeader {
        ua::RequestHeader::raw_mut(&mut self.0.requestHeader)
    }
}
