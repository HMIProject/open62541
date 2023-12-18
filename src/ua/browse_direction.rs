use std::fmt;

use open62541_sys::UA_BrowseDirection;

/// Wrapper for browse direction from [`open62541_sys`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BrowseDirection(UA_BrowseDirection);

impl BrowseDirection {
    #[must_use]
    pub const fn forward() -> Self {
        Self(UA_BrowseDirection::UA_BROWSEDIRECTION_FORWARD)
    }

    #[must_use]
    pub const fn inverse() -> Self {
        Self(UA_BrowseDirection::UA_BROWSEDIRECTION_INVERSE)
    }

    #[must_use]
    pub const fn both() -> Self {
        Self(UA_BrowseDirection::UA_BROWSEDIRECTION_BOTH)
    }

    #[must_use]
    pub const fn invalid() -> Self {
        Self(UA_BrowseDirection::UA_BROWSEDIRECTION_INVALID)
    }

    /// Creates wrapper by taking ownership of `src`.
    #[allow(dead_code)]
    #[must_use]
    pub(crate) const fn new(src: UA_BrowseDirection) -> Self {
        Self(src)
    }

    /// Gives up ownership and returns inner value.
    #[must_use]
    pub(crate) const fn into_inner(self) -> UA_BrowseDirection {
        self.0
    }

    #[allow(clippy::unnecessary_cast)]
    #[must_use]
    pub(crate) const fn as_u32(&self) -> u32 {
        // This cast is necessary on Windows builds where `UA_BrowseDirection` wraps an `i32`.
        (self.0).0 as u32
    }
}

impl fmt::Display for BrowseDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self.0 {
            UA_BrowseDirection::UA_BROWSEDIRECTION_FORWARD => "FORWARD",
            UA_BrowseDirection::UA_BROWSEDIRECTION_INVERSE => "INVERSE",
            UA_BrowseDirection::UA_BROWSEDIRECTION_BOTH => "BOTH",
            UA_BrowseDirection::UA_BROWSEDIRECTION_INVALID => "INVALID",
            _ => "?",
        };
        f.write_str(str)
    }
}
