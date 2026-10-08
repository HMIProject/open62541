use crate::{ServiceResponse, ua};

crate::data_type!(WriteResponse);
crate::member_accessors!(WriteResponse {
    results: [ua::StatusCode],
    responseHeader: &ua::ResponseHeader
});

impl ServiceResponse for WriteResponse {
    type Request = ua::WriteRequest;

    fn response_header(&self) -> &ua::ResponseHeader {
        Self::response_header(self)
    }
}
