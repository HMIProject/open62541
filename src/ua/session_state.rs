use open62541_sys::UA_SessionState;

/// Wrapper for [`UA_SessionState`] from [`open62541_sys`].
#[derive(Debug)]
pub struct SessionState(UA_SessionState);

impl SessionState {
    /// Creates wrapper initialized with defaults.
    #[must_use]
    pub(crate) const fn init() -> Self {
        // Use default variant that corresponds to numeric value `0` to match other `init()` calls.
        Self(UA_SessionState::UA_SESSIONSTATE_CLOSED)
    }

    /// Returns mutable pointer to value.
    #[must_use]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut UA_SessionState {
        &mut self.0
    }
}
