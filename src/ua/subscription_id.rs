/// Wrapper for subscription IDs from [`open62541_sys`].
#[derive(Clone, Copy, Debug)]
pub struct SubscriptionId(u32);

impl SubscriptionId {
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
