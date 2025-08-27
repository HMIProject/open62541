use crate::{ua, DataType as _, ServiceRequest};

crate::data_type!(ReadRequest);

impl ReadRequest {
    #[must_use]
    pub fn with_timestamps_to_return(
        mut self,
        timestamps_to_return: &ua::TimestampsToReturn,
    ) -> Self {
        timestamps_to_return.clone_into_raw(&mut self.0.timestampsToReturn);
        self
    }

    #[must_use]
    pub fn with_nodes_to_read(mut self, nodes_to_read: &[ua::ReadValueId]) -> Self {
        let array = ua::Array::from_slice(nodes_to_read);
        array.move_into_raw(&mut self.0.nodesToReadSize, &mut self.0.nodesToRead);
        self
    }
}

impl ServiceRequest for ReadRequest {
    type Response = ua::ReadResponse;

    fn request_header(&self) -> &ua::RequestHeader {
        ua::RequestHeader::raw_ref(&self.0.requestHeader)
    }

    fn request_header_mut(&mut self) -> &mut ua::RequestHeader {
        ua::RequestHeader::raw_mut(&mut self.0.requestHeader)
    }
}
