use crate::{DataType as _, ServiceResponse, ua};

crate::data_type!(BrowseNextResponse);
crate::member_accessors!(BrowseNextResponse {
    results: [ua::BrowseResult],
    responseHeader: &ua::ResponseHeader
});

impl ServiceResponse for BrowseNextResponse {
    type Request = ua::BrowseNextRequest;

    fn response_header(&self) -> &ua::ResponseHeader {
        ua::ResponseHeader::raw_ref(&self.0.responseHeader)
    }
}
