use crate::{ua, ServiceRequest};

crate::data_type!(WriteRequest);

impl WriteRequest {
    #[must_use]
    pub fn with_nodes_to_write(mut self, nodes_to_write: &[ua::WriteValue]) -> Self {
        let array = ua::Array::from_slice(nodes_to_write);
        array.move_into(&mut self.0.nodesToWriteSize, &mut self.0.nodesToWrite);
        self
    }
}

impl ServiceRequest for WriteRequest {
    type Response = ua::WriteResponse;
}
