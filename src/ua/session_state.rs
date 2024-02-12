use open62541_sys::UA_SessionState;

/// Wrapper for [`UA_SessionState`] from [`open62541_sys`].
pub struct SessionState(UA_SessionState);

impl SessionState {
    /// Creates wrapper initialized with defaults.
    #[must_use]
    pub(crate) const fn init() -> Self {
        Self(UA_SessionState::UA_SESSIONSTATE_CLOSED)
    }

    /// Returns mutable pointer to value.
    #[must_use]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut UA_SessionState {
        &mut self.0
    }
}
