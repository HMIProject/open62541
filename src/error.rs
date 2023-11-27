use open62541_sys::UA_STATUSCODE_GOOD;
use thiserror::Error;

use crate::ua;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Error)]
#[error("{0}")]
pub struct Error(ua::StatusCode);

impl Error {
    #[must_use]
    pub fn new(status_code: u32) -> Self {
        debug_assert_ne!(status_code, UA_STATUSCODE_GOOD);
        Self(ua::StatusCode::new(status_code))
    }
}
