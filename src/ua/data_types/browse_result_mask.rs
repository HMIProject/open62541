use open62541_sys::UA_BrowseResultMask;

crate::data_type!(BrowseResultMask);

crate::enum_variants!(
    BrowseResultMask,
    UA_BrowseResultMask,
    [
        NONE,
        REFERENCETYPEID,
        ISFORWARD,
        NODECLASS,
        BROWSENAME,
        DISPLAYNAME,
        TYPEDEFINITION,
        ALL,
        REFERENCETYPEINFO,
        TARGETINFO,
    ],
);

// TODO: Support bit operations on this mask.
impl BrowseResultMask {
    #[deprecated(note = "use `Self::NONE` instead")]
    #[must_use]
    pub const fn none() -> Self {
        Self(UA_BrowseResultMask::UA_BROWSERESULTMASK_NONE)
    }

    #[deprecated(note = "use `Self::REFERENCETYPEID` instead")]
    #[must_use]
    pub const fn reference_type_id() -> Self {
        Self(UA_BrowseResultMask::UA_BROWSERESULTMASK_REFERENCETYPEID)
    }

    #[deprecated(note = "use `Self::ISFORWARD` instead")]
    #[must_use]
    pub const fn is_forward() -> Self {
        Self(UA_BrowseResultMask::UA_BROWSERESULTMASK_ISFORWARD)
    }

    #[deprecated(note = "use `Self::NODECLASS` instead")]
    #[must_use]
    pub const fn node_class() -> Self {
        Self(UA_BrowseResultMask::UA_BROWSERESULTMASK_NODECLASS)
    }

    #[deprecated(note = "use `Self::BROWSENAME` instead")]
    #[must_use]
    pub const fn browse_name() -> Self {
        Self(UA_BrowseResultMask::UA_BROWSERESULTMASK_BROWSENAME)
    }

    #[deprecated(note = "use `Self::DISPLAYNAME` instead")]
    #[must_use]
    pub const fn display_name() -> Self {
        Self(UA_BrowseResultMask::UA_BROWSERESULTMASK_DISPLAYNAME)
    }

    #[deprecated(note = "use `Self::TYPEDEFINITION` instead")]
    #[must_use]
    pub const fn type_definition() -> Self {
        Self(UA_BrowseResultMask::UA_BROWSERESULTMASK_TYPEDEFINITION)
    }

    #[deprecated(note = "use `Self::ALL` instead")]
    #[must_use]
    pub const fn all() -> Self {
        Self(UA_BrowseResultMask::UA_BROWSERESULTMASK_ALL)
    }

    #[deprecated(note = "use `Self::REFERENCETYPEINFO` instead")]
    #[must_use]
    pub const fn reference_type_info() -> Self {
        Self(UA_BrowseResultMask::UA_BROWSERESULTMASK_REFERENCETYPEINFO)
    }

    #[deprecated(note = "use `Self::TARGETINFO` instead")]
    #[must_use]
    pub const fn target_info() -> Self {
        Self(UA_BrowseResultMask::UA_BROWSERESULTMASK_TARGETINFO)
    }
}
