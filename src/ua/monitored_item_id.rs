/// Wrapper for monitored item ID from [`open62541_sys`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MonitoredItemId(u32);

impl MonitoredItemId {
    /// Creates wrapper by taking ownership of `src`.
    #[must_use]
    pub(crate) const fn new(src: u32) -> Self {
        Self(src)
    }

    /// Gives up ownership and returns inner value.
    #[must_use]
    pub(crate) const fn into_inner(self) -> u32 {
        self.0
    }
}
