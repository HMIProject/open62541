use crate::{DataType as _, ServiceRequest, ua};

crate::data_type!(HistoryReadRequest);

impl HistoryReadRequest {
    #[must_use]
    pub fn with_timestamps_to_return(
        mut self,
        timestamps_to_return: &ua::TimestampsToReturn,
    ) -> Self {
        timestamps_to_return.clone_into_raw(&mut self.0.timestampsToReturn);
        self
    }

    #[must_use]
    pub fn with_nodes_to_read(mut self, nodes_to_read: &[ua::HistoryReadValueId]) -> Self {
        let array = ua::Array::from_slice(nodes_to_read);
        array.move_into_raw(&mut self.0.nodesToReadSize, &mut self.0.nodesToRead);
        self
    }

    #[must_use]
    pub fn with_history_read_details(
        mut self,
        history_read_details: &ua::ReadRawModifiedDetails,
    ) -> Self {
        history_read_details
            .to_extension_object()
            .clone_into_raw(&mut self.0.historyReadDetails);
        self
    }
}

impl ServiceRequest for HistoryReadRequest {
    type Response = ua::HistoryReadResponse;

    fn request_header(&self) -> &ua::RequestHeader {
        ua::RequestHeader::raw_ref(&self.0.requestHeader)
    }

    fn request_header_mut(&mut self) -> &mut ua::RequestHeader {
        ua::RequestHeader::raw_mut(&mut self.0.requestHeader)
    }
}
