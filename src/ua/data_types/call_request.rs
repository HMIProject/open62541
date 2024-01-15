use crate::{ua, ServiceRequest};

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
}
