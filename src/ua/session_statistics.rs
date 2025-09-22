use open62541_sys::UA_SessionStatistics;

#[derive(Debug)]
#[repr(transparent)]
pub struct SessionStatistics(UA_SessionStatistics);

impl SessionStatistics {
    /// Creates wrapper reference from value.
    #[must_use]
    pub(crate) const fn raw_ref(src: &UA_SessionStatistics) -> &Self {
        let src: *const UA_SessionStatistics = src;
        // This transmutes between the inner type and `Self` through `cast()`. This is okay because
        // we are using `#[repr(transparent)]`.
        let ptr = src.cast::<Self>();
        // SAFETY: Pointer is valid and allowed to reference `Self` due to `#[repr(transparent)]`.
        let ptr = unsafe { ptr.as_ref() };
        // SAFETY: Pointer is valid (non-zero) because it comes from a reference.
        unsafe { ptr.unwrap_unchecked() }
    }

    #[must_use]
    pub const fn current_session_count(&self) -> usize {
        self.0.currentSessionCount
    }

    #[must_use]
    pub const fn cumulated_session_count(&self) -> usize {
        self.0.cumulatedSessionCount
    }

    #[must_use]
    pub const fn security_rejected_session_count(&self) -> usize {
        self.0.securityRejectedSessionCount
    }

    #[must_use]
    pub const fn rejected_session_count(&self) -> usize {
        self.0.rejectedSessionCount
    }

    #[must_use]
    pub const fn session_timeout_count(&self) -> usize {
        self.0.sessionTimeoutCount
    }

    #[must_use]
    pub const fn session_abort_count(&self) -> usize {
        self.0.sessionAbortCount
    }
}
