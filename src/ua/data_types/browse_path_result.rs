use crate::data_type::DataType;
use crate::ua;

crate::data_type!(BrowsePathResult);

impl BrowsePathResult {
    #[must_use]
    pub const fn get_status_code(&self) -> ua::StatusCode {
        ua::StatusCode::new(self.0.statusCode)
    }

    #[must_use]
    pub const fn get_targets_size(&self) -> usize {
        self.0.targetsSize
    }

    /// # Panics
    ///
    /// This panics if the giving index could not be accessed.
    #[must_use]
    pub fn get_target(&self, index: usize) -> ua::BrowsePathTarget {
        unsafe {
            ua::BrowsePathTarget::clone_raw(
                self.0
                    .targets
                    .wrapping_add(index)
                    .as_mut()
                    .expect("Invalid target accessed!"),
            )
        }
    }
}
