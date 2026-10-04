use open62541_sys::UA_BrowsePathResult;

use crate::ua;

crate::data_type!(BrowsePathResult);

impl BrowsePathResult {
    #[must_use]
    pub const fn status_code(&self) -> ua::StatusCode {
        ua::StatusCode::new(self.0.statusCode)
    }

    #[must_use]
    pub fn into_targets(mut self) -> Option<ua::Array<ua::BrowsePathTarget>> {
        unsafe { ua::Array::move_from_raw_parts(&mut self.0.targetsSize, &mut self.0.targets) }
    }

    #[must_use]
    pub fn targets(&self) -> Option<&[ua::BrowsePathTarget]> {
        unsafe { ua::Array::slice_from_raw_parts(self.0.targetsSize, self.0.targets) }
    }
}
