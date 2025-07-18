use std::fmt;

use crate::ua;

/// Wrapper for monitored item ID from [`open62541_sys`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MonitoredItemId(u32);

impl MonitoredItemId {
    #[must_use]
    pub(crate) const fn new(id: u32) -> Self {
        Self(id)
    }

    #[must_use]
    pub(crate) const fn as_u32(self) -> u32 {
        self.0
    }

    #[must_use]
    pub(crate) const fn to_uint32(self) -> ua::UInt32 {
        ua::UInt32::new(self.as_u32())
    }
}

impl fmt::Display for MonitoredItemId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
