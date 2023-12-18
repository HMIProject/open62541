use std::fmt;

use open62541_sys::UA_BrowseResultMask;

/// Wrapper for result masks from [`open62541_sys`].
#[derive(Clone, Debug)]
pub struct BrowseResultMask(UA_BrowseResultMask);

// TODO: Support bit operations on this mask.
impl BrowseResultMask {
    #[must_use]
    pub const fn none() -> Self {
        Self(UA_BrowseResultMask::UA_BROWSERESULTMASK_NONE)
    }

    #[must_use]
    pub const fn reference_type_id() -> Self {
        Self(UA_BrowseResultMask::UA_BROWSERESULTMASK_REFERENCETYPEID)
    }

    #[must_use]
    pub const fn is_forward() -> Self {
        Self(UA_BrowseResultMask::UA_BROWSERESULTMASK_ISFORWARD)
    }

    #[must_use]
    pub const fn node_class() -> Self {
        Self(UA_BrowseResultMask::UA_BROWSERESULTMASK_NODECLASS)
    }

    #[must_use]
    pub const fn browse_name() -> Self {
        Self(UA_BrowseResultMask::UA_BROWSERESULTMASK_BROWSENAME)
    }

    #[must_use]
    pub const fn display_name() -> Self {
        Self(UA_BrowseResultMask::UA_BROWSERESULTMASK_DISPLAYNAME)
    }

    #[must_use]
    pub const fn type_definition() -> Self {
        Self(UA_BrowseResultMask::UA_BROWSERESULTMASK_TYPEDEFINITION)
    }

    #[must_use]
    pub const fn all() -> Self {
        Self(UA_BrowseResultMask::UA_BROWSERESULTMASK_ALL)
    }

    #[must_use]
    pub const fn reference_type_info() -> Self {
        Self(UA_BrowseResultMask::UA_BROWSERESULTMASK_REFERENCETYPEINFO)
    }

    #[must_use]
    pub const fn target_info() -> Self {
        Self(UA_BrowseResultMask::UA_BROWSERESULTMASK_TARGETINFO)
    }

    /// Creates wrapper by taking ownership of `src`.
    #[allow(dead_code)]
    #[must_use]
    pub(crate) const fn new(src: UA_BrowseResultMask) -> Self {
        Self(src)
    }

    /// Gives up ownership and returns inner value.
    #[must_use]
    pub(crate) const fn into_inner(self) -> UA_BrowseResultMask {
        self.0
    }

    #[allow(clippy::unnecessary_cast)]
    #[must_use]
    pub(crate) const fn as_u32(&self) -> u32 {
        // This cast is necessary on Windows builds where `UA_BrowseResultMask` wraps an `i32`.
        (self.0).0 as u32
    }
}

impl fmt::Display for BrowseResultMask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Handle bit combinations on this mask.
        let str = match self.0 {
            UA_BrowseResultMask::UA_BROWSERESULTMASK_NONE => "NONE",
            UA_BrowseResultMask::UA_BROWSERESULTMASK_REFERENCETYPEID => "REFERENCETYPEID",
            UA_BrowseResultMask::UA_BROWSERESULTMASK_ISFORWARD => "ISFORWARD",
            UA_BrowseResultMask::UA_BROWSERESULTMASK_NODECLASS => "NODECLASS",
            UA_BrowseResultMask::UA_BROWSERESULTMASK_BROWSENAME => "BROWSENAME",
            UA_BrowseResultMask::UA_BROWSERESULTMASK_DISPLAYNAME => "DISPLAYNAME",
            UA_BrowseResultMask::UA_BROWSERESULTMASK_TYPEDEFINITION => "TYPEDEFINITION",
            UA_BrowseResultMask::UA_BROWSERESULTMASK_ALL => "ALL",
            UA_BrowseResultMask::UA_BROWSERESULTMASK_REFERENCETYPEINFO => "REFERENCETYPEINFO",
            UA_BrowseResultMask::UA_BROWSERESULTMASK_TARGETINFO => "TARGETINFO",
            _ => "?",
        };
        f.write_str(str)
    }
}
