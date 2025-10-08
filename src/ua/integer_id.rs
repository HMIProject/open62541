use std::fmt;

use crate::ua;

/// Wrapper for an [`IntegerId`](https://reference.opcfoundation.org/Core/Part4/v105/docs/7.19).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IntegerId(u32);

impl IntegerId {
    pub(crate) const INVALID: Self = Self(0);

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

    /// Checks if the id is either valid or invalid/undefined.
    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.0 != Self::INVALID.0
    }
}

impl fmt::Display for IntegerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
