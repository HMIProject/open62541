use open62541_sys::UA_SecureChannelState;

/// Wrapper for [`UA_SecureChannelState`] from [`open62541_sys`].
pub struct SecureChannelState(UA_SecureChannelState);

impl SecureChannelState {
    /// Creates wrapper initialized with defaults.
    #[must_use]
    #[allow(dead_code)] // --no-default-features
    pub(crate) const fn init() -> Self {
        Self(UA_SecureChannelState::UA_SECURECHANNELSTATE_CLOSED)
    }

    /// Returns mutable pointer to value.
    #[must_use]
    #[allow(dead_code)] // --no-default-features
    pub(crate) fn as_mut_ptr(&mut self) -> *mut UA_SecureChannelState {
        &mut self.0
    }
}
