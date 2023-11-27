use crate::ua;

ua::data_type!(ReadRequest, UA_ReadRequest, UA_TYPES_READREQUEST);

impl ReadRequest {
    #[must_use]
    pub fn nodes_to_read(mut self, nodes_to_read: &[ua::ReadValueId]) -> Self {
        let array = ua::Array::from_slice(nodes_to_read);

        let (size, ptr) = array.into_raw_parts();
        self.0.nodesToReadSize = size;
        self.0.nodesToRead = ptr;

        self
    }
}
