use crate::{DataType as _, ServiceResponse, ua};

crate::data_type!(ReadResponse);
crate::member_accessors!(ReadResponse {
    results: [ua::DataValue],
    responseHeader: &ua::ResponseHeader
});

impl ServiceResponse for ReadResponse {
    type Request = ua::ReadRequest;

    fn response_header(&self) -> &ua::ResponseHeader {
        ua::ResponseHeader::raw_ref(&self.0.responseHeader)
    }
}
