use crate::ua;

crate::data_type!(ReadRequest, UA_ReadRequest, UA_TYPES_READREQUEST);

impl ReadRequest {
    #[must_use]
    pub fn with_nodes_to_read(mut self, nodes_to_read: &[ua::ReadValueId]) -> Self {
        let array = ua::Array::from_slice(nodes_to_read);

        // This transfers ownership from local variable `array` into `self`.
        let (size, ptr) = array.into_raw_parts();
        self.0.nodesToReadSize = size;
        self.0.nodesToRead = ptr;

        self
    }
}
