use open62541_sys::UA_BrowseResultMask;

/// Wrapper for result masks from [`open62541_sys`].
#[derive(Clone, Debug)]
pub struct ResultMask(UA_BrowseResultMask);

impl ResultMask {
    #[must_use]
    pub const fn none() -> Self {
        Self(UA_BrowseResultMask::UA_BROWSERESULTMASK_NONE)
    }

    #[must_use]
    pub const fn reference_type_info() -> Self {
        Self(UA_BrowseResultMask::UA_BROWSERESULTMASK_REFERENCETYPEINFO)
    }

    #[must_use]
    pub const fn target_info() -> Self {
        Self(UA_BrowseResultMask::UA_BROWSERESULTMASK_TARGETINFO)
    }

    #[must_use]
    pub const fn all() -> Self {
        Self(UA_BrowseResultMask::UA_BROWSERESULTMASK_ALL)
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
}
