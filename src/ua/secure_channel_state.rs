use open62541_sys::UA_SecureChannelState;

/// Wrapper for [`UA_SecureChannelState`] from [`open62541_sys`].
pub struct SecureChannelState(UA_SecureChannelState);

impl SecureChannelState {
    /// Creates wrapper initialized with defaults.
    #[must_use]
    pub(crate) const fn init() -> Self {
        Self(UA_SecureChannelState::UA_SECURECHANNELSTATE_FRESH)
    }

    /// Returns mutable pointer to value.
    #[must_use]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut UA_SecureChannelState {
        &mut self.0
    }
}
