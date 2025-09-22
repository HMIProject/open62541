use crate::{DataType as _, ServiceRequest, ua};

crate::data_type!(WriteRequest);

impl WriteRequest {
    #[must_use]
    pub fn with_nodes_to_write(mut self, nodes_to_write: &[ua::WriteValue]) -> Self {
        let array = ua::Array::from_slice(nodes_to_write);
        array.move_into_raw(&mut self.0.nodesToWriteSize, &mut self.0.nodesToWrite);
        self
    }
}

impl ServiceRequest for WriteRequest {
    type Response = ua::WriteResponse;

    fn request_header(&self) -> &ua::RequestHeader {
        ua::RequestHeader::raw_ref(&self.0.requestHeader)
    }

    fn request_header_mut(&mut self) -> &mut ua::RequestHeader {
        ua::RequestHeader::raw_mut(&mut self.0.requestHeader)
    }
}
