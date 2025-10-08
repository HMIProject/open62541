use std::fmt;

use crate::ua;

/// Wrapper for subscription ID from [`open62541_sys`].
///
/// Newtype wrapper for [`ua::IntegerId`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SubscriptionId(ua::IntegerId);

impl SubscriptionId {
    #[must_use]
    pub(crate) const fn new(id: ua::IntegerId) -> Self {
        Self(id)
    }

    /// Gets the generic [`ua::IntegerId`].
    #[must_use]
    pub const fn as_id(self) -> ua::IntegerId {
        self.0
    }
}

impl fmt::Display for SubscriptionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
