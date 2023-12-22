use crate::{ua, ServiceRequest};

crate::data_type!(ReadRequest);

impl ReadRequest {
    #[must_use]
    pub fn with_nodes_to_read(mut self, nodes_to_read: &[ua::ReadValueId]) -> Self {
        let array = ua::Array::from_slice(nodes_to_read);
        array.move_into(&mut self.0.nodesToReadSize, &mut self.0.nodesToRead);
        self
    }
}

impl ServiceRequest for ReadRequest {
    type Response = ua::ReadResponse;
}
