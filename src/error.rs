use thiserror::Error;

use crate::ua;

/// Result type used in this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Error type used in this crate.
///
/// This error may be returned from many different OPC UA calls. It represents any status code
/// that doesn't qualify as [`is_good()`].
///
/// [`is_good()`]: crate::ua::StatusCode::is_good
#[derive(Debug, Error)]
pub enum Error {
    /// Error from server.
    #[error("{0}")]
    Server(ua::StatusCode),

    /// Internal error.
    #[error("{0}")]
    Internal(&'static str),
}

impl Error {
    /// Creates error from status code.
    ///
    /// To avoid confusion, the status code should be considered bad or uncertain but not good, as
    /// indicated by [`ua::StatusCode::is_good()`].
    #[must_use]
    pub(crate) fn new(status_code: ua::StatusCode) -> Self {
        debug_assert!(!status_code.is_good());
        Self::Server(status_code)
    }

    pub(crate) fn verify_good(status_code: &ua::StatusCode) -> Result<()> {
        if status_code.is_good() {
            Ok(())
        } else {
            Err(Self::new(status_code.clone()))
        }
    }

    /// Gets associated OPC UA status code.
    ///
    /// This returns the original status code except for internal errors where the generic status
    /// code [`ua::StatusCode::BAD`] is returned instead.
    #[must_use]
    pub fn status_code(&self) -> ua::StatusCode {
        match self {
            // TODO: Avoid clone and make `ua::StatusCode` derive `Copy`.
            Error::Server(status_code) => status_code.clone(),
            Error::Internal(_) => ua::StatusCode::BAD,
        }
    }

    #[allow(dead_code)] // --no-default-features
    #[must_use]
    pub(crate) const fn internal(message: &'static str) -> Self {
        Self::Internal(message)
    }
}
