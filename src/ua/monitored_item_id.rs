use crate::ua;

/// Wrapper for monitored item ID from [`open62541_sys`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MonitoredItemId(u32);

impl MonitoredItemId {
    #[must_use]
    pub(crate) const fn new(id: u32) -> Self {
        Self(id)
    }

    pub(crate) const fn as_u32(self) -> u32 {
        self.0
    }

    pub(crate) fn to_uint32(self) -> ua::UInt32 {
        ua::UInt32::new(self.as_u32())
    }
}
