use open62541_sys::UA_STATUSCODE_GOOD;
use thiserror::Error;

use crate::ua;

/// Generic error.
///
/// This error may be returned from many different OPC UA calls. It represents any status code other
/// than [`UA_STATUSCODE_GOOD`].
#[derive(Debug, Error)]
#[error("{0}")]
pub enum Error {
    /// Error from server.
    Server(ua::StatusCode),
    /// Internal error.
    Internal(&'static str),
}

impl Error {
    #[must_use]
    pub(crate) fn new(status_code: u32) -> Self {
        debug_assert_ne!(status_code, UA_STATUSCODE_GOOD);
        Self::Server(ua::StatusCode::new(status_code))
    }

    pub(crate) fn verify_good(status_code: u32) -> Result<(), Self> {
        if status_code == UA_STATUSCODE_GOOD {
            Ok(())
        } else {
            Err(Self::new(status_code))
        }
    }

    #[must_use]
    pub(crate) const fn internal(message: &'static str) -> Self {
        Self::Internal(message)
    }
}
