use crate::{DataType as _, ua};

crate::data_type!(CallMethodRequest);
crate::member_accessors!(CallMethodRequest {
    objectId: &ua::NodeId,
    methodId: &ua::NodeId,
    inputArguments: [ua::Variant]
});

impl CallMethodRequest {
    #[must_use]
    pub fn with_object_id(mut self, object_id: &ua::NodeId) -> Self {
        object_id.clone_into_raw(&mut self.0.objectId);
        self
    }

    #[must_use]
    pub fn with_method_id(mut self, method_id: &ua::NodeId) -> Self {
        method_id.clone_into_raw(&mut self.0.methodId);
        self
    }

    #[must_use]
    pub fn with_input_arguments(mut self, input_arguments: &[ua::Variant]) -> Self {
        let array = ua::Array::from_slice(input_arguments);
        array.move_into_raw(&mut self.0.inputArgumentsSize, &mut self.0.inputArguments);
        self
    }
}
