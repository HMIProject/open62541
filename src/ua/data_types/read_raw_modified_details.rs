use crate::{DataType as _, ua};

crate::data_type!(ReadRawModifiedDetails);

impl ReadRawModifiedDetails {
    #[must_use]
    pub fn with_is_read_modified(mut self, is_read_modified: ua::Boolean) -> Self {
        is_read_modified.move_into_raw(&mut self.0.isReadModified);
        self
    }

    #[must_use]
    pub fn with_start_time(mut self, start_time: ua::DateTime) -> Self {
        start_time.move_into_raw(&mut self.0.startTime);
        self
    }

    #[must_use]
    pub fn with_end_time(mut self, end_time: ua::DateTime) -> Self {
        end_time.move_into_raw(&mut self.0.endTime);
        self
    }

    #[must_use]
    pub fn with_num_values_per_node(mut self, num_values_per_node: ua::UInt32) -> Self {
        num_values_per_node.move_into_raw(&mut self.0.numValuesPerNode);
        self
    }

    #[must_use]
    pub fn with_return_bounds(mut self, return_bounds: ua::Boolean) -> Self {
        return_bounds.move_into_raw(&mut self.0.returnBounds);
        self
    }

    #[must_use]
    pub fn to_extension_object(&self) -> ua::ExtensionObject {
        ua::ExtensionObject::new(self)
    }
}
