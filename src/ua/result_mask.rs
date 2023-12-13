use open62541_sys::{
    UA_BrowseResultMask_UA_BROWSERESULTMASK_ALL, UA_BrowseResultMask_UA_BROWSERESULTMASK_NONE,
    UA_BrowseResultMask_UA_BROWSERESULTMASK_REFERENCETYPEINFO,
    UA_BrowseResultMask_UA_BROWSERESULTMASK_TARGETINFO,
};

/// Wrapper for result masks from [`open62541_sys`].
#[derive(Clone, Copy, Debug)]
pub struct ResultMask(u32);

impl ResultMask {
    /// Creates wrapper by taking ownership of `src`.
    #[allow(dead_code)]
    #[must_use]
    pub(crate) const fn new(src: u32) -> Self {
        Self(src)
    }

    #[must_use]
    pub const fn none() -> Self {
        Self(UA_BrowseResultMask_UA_BROWSERESULTMASK_NONE)
    }

    #[must_use]
    pub const fn reference_type_info() -> Self {
        Self(UA_BrowseResultMask_UA_BROWSERESULTMASK_REFERENCETYPEINFO)
    }

    #[must_use]
    pub const fn target_info() -> Self {
        Self(UA_BrowseResultMask_UA_BROWSERESULTMASK_TARGETINFO)
    }

    #[must_use]
    pub const fn all() -> Self {
        Self(UA_BrowseResultMask_UA_BROWSERESULTMASK_ALL)
    }

    /// Gives up ownership and returns inner value.
    #[must_use]
    pub(crate) const fn into_inner(self) -> u32 {
        self.0
    }
}
