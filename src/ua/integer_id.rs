use std::{fmt, num::NonZeroU32};

use crate::ua;

/// Wrapper for a valid [`IntegerId`](https://reference.opcfoundation.org/Core/Part4/v105/docs/7.19).
///
/// Only stores valid, non-zero values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct IntegerId(NonZeroU32);

impl IntegerId {
    #[must_use]
    pub(crate) const fn new(id: NonZeroU32) -> Self {
        Self(id)
    }

    #[must_use]
    pub(crate) const fn from_u32(id: u32) -> Option<Self> {
        if let Some(id) = NonZeroU32::new(id) {
            Some(Self::new(id))
        } else {
            None
        }
    }

    #[must_use]
    pub(crate) const fn as_u32(self) -> u32 {
        self.0.get()
    }

    #[must_use]
    pub(crate) const fn to_uint32(self) -> ua::UInt32 {
        ua::UInt32::new(self.as_u32())
    }
}

impl fmt::Display for IntegerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
