use open62541_sys::UA_SecureChannelStatistics;

#[derive(Debug)]
#[repr(transparent)]
pub struct SecureChannelStatistics(UA_SecureChannelStatistics);

impl SecureChannelStatistics {
    /// Creates wrapper reference from value.
    #[must_use]
    pub(crate) fn raw_ref(src: &UA_SecureChannelStatistics) -> &Self {
        let src: *const UA_SecureChannelStatistics = src;
        // This transmutes between the inner type and `Self` through `cast()`. This is okay because
        // we are using `#[repr(transparent)]`.
        let ptr = src.cast::<Self>();
        // SAFETY: Pointer is valid and allowed to reference `Self` due to `#[repr(transparent)]`.
        let ptr = unsafe { ptr.as_ref() };
        // SAFETY: Pointer is valid (non-zero) because it comes from a reference.
        unsafe { ptr.unwrap_unchecked() }
    }

    #[must_use]
    pub const fn current_channel_count(&self) -> usize {
        self.0.currentChannelCount
    }

    #[must_use]
    pub const fn cumulated_channel_count(&self) -> usize {
        self.0.cumulatedChannelCount
    }

    #[must_use]
    pub const fn rejected_channel_count(&self) -> usize {
        self.0.rejectedChannelCount
    }

    #[must_use]
    pub const fn channel_timeout_count(&self) -> usize {
        self.0.channelTimeoutCount
    }

    #[must_use]
    pub const fn channel_abort_count(&self) -> usize {
        self.0.channelAbortCount
    }

    #[must_use]
    pub const fn channel_purge_count(&self) -> usize {
        self.0.channelPurgeCount
    }
}
