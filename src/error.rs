use open62541_sys::UA_STATUSCODE_GOOD;
use thiserror::Error;

use crate::ua;

/// Generic error.
///
/// This error may be returned from many different OPC UA calls. It represents any status code other
/// than [`UA_STATUSCODE_GOOD`].
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
