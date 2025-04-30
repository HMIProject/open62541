use crate::{ua, DataType as _};

crate::data_type!(ServerStatusDataType);

impl ServerStatusDataType {
    #[must_use]
    pub fn build_info(&self) -> &ua::BuildInfo {
        ua::BuildInfo::raw_ref(&self.0.buildInfo)
    }

    #[must_use]
    pub fn state(&self) -> &ua::ServerState {
        ua::ServerState::raw_ref(&self.0.state)
    }

    #[must_use]
    pub fn start_time(&self) -> ua::DateTime {
        // SAFETY: The i64 value represents a valid UtcTime.
        unsafe { ua::DateTime::from_raw(self.0.startTime) }
    }

    #[must_use]
    pub fn current_time(&self) -> ua::DateTime {
        // SAFETY: The i64 value represents a valid UtcTime.
        unsafe { ua::DateTime::from_raw(self.0.currentTime) }
    }

    #[must_use]
    pub const fn seconds_till_shutdown(&self) -> u32 {
        self.0.secondsTillShutdown
    }

    #[must_use]
    pub fn shutdown_reason(&self) -> &ua::LocalizedText {
        ua::LocalizedText::raw_ref(&self.0.shutdownReason)
    }
}
