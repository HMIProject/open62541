use open62541_sys::UA_SecureChannelState;

/// Wrapper for [`UA_SecureChannelState`] from [`open62541_sys`].
#[derive(Debug)]
pub struct SecureChannelState(UA_SecureChannelState);

impl SecureChannelState {
    /// Creates wrapper initialized with defaults.
    #[must_use]
    pub(crate) const fn init() -> Self {
        // Use default variant that corresponds to numeric value `0` to match other `init()` calls.
        Self(UA_SecureChannelState::UA_SECURECHANNELSTATE_CLOSED)
    }

    /// Returns mutable pointer to value.
    #[must_use]
    #[expect(clippy::allow_attributes, reason = "non-static condition")]
    #[allow(clippy::missing_const_for_fn, reason = "unsupported before Rust 1.87")]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut UA_SecureChannelState {
        &raw mut self.0
    }
}
