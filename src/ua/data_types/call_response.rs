use crate::{ServiceResponse, ua};

crate::data_type!(CallResponse);
crate::member_accessors!(CallResponse {
    results: [ua::CallMethodResult],
    responseHeader: &ua::ResponseHeader
});

impl ServiceResponse for CallResponse {
    type Request = ua::CallRequest;

    fn response_header(&self) -> &ua::ResponseHeader {
        Self::response_header(self)
    }
}
