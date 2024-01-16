use thiserror::Error;

use crate::ua;

/// Generic error.
///
/// This error may be returned from many different OPC UA calls. It represents any status code other
/// than [`UA_STATUSCODE_GOOD`].
///
/// [`UA_STATUSCODE_GOOD`]: open62541_sys::UA_STATUSCODE_GOOD
#[derive(Debug, Error)]
pub enum Error {
    /// Error from server.
    #[error("{0:?}")]
    Server(ua::StatusCode),
    /// Internal error.
    #[error("{0}")]
    Internal(&'static str),
}

impl Error {
    #[must_use]
    pub(crate) fn new(status_code: ua::StatusCode) -> Self {
        debug_assert!(!status_code.is_good());
        Self::Server(status_code)
    }

    pub(crate) fn verify_good(status_code: &ua::StatusCode) -> Result<(), Self> {
        if status_code.is_good() {
            Ok(())
        } else {
            Err(Self::new(status_code.clone()))
        }
    }

    #[allow(dead_code)] // Temporarily lift linter warning for `--no-default-features`.
    #[must_use]
    pub(crate) const fn internal(message: &'static str) -> Self {
        Self::Internal(message)
    }
}
