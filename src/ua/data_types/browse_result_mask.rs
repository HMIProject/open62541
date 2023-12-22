use open62541_sys::UA_BrowseResultMask;

crate::data_type!(BrowseResultMask);

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

    pub(crate) fn as_u32(&self) -> u32 {
        // This cast is necessary on Windows builds with inner type `i32`.
        #[allow(clippy::useless_conversion)]
        u32::try_from((self.0).0).expect("should convert to u32")
    }
}
