use std::fmt;

use crate::ua;

/// Wrapper for monitored item ID from [`open62541_sys`].
///
/// Newtype wrapper for [`ua::IntegerId`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MonitoredItemId(ua::IntegerId);

impl MonitoredItemId {
    #[must_use]
    pub(crate) const fn new(id: ua::IntegerId) -> Self {
        Self(id)
    }

    /// Gets the generic [`ua::IntegerId`].
    #[must_use]
    pub(crate) const fn as_id(self) -> ua::IntegerId {
        self.0
    }
}

impl fmt::Display for MonitoredItemId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
