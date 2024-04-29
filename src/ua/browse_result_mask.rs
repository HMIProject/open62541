// We do not expose the inner enum. We want to use a proper `u32` for bit operations on the mask and
// we want to be clear about what is an initial (const, enum-like) value and what is a derived mask;
// specifically, the bitmask type is _not_ an enum even though declared so in `open62541-sys`.
mod inner {
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
}

/// Wrapper for browse result mask from [`open62541_sys`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BrowseResultMask(u32);

crate::bitmask_ops!(BrowseResultMask);

impl BrowseResultMask {
    pub const NONE: Self = Self(inner::BrowseResultMask::NONE_U32);
    pub const REFERENCETYPEID: Self = Self(inner::BrowseResultMask::REFERENCETYPEID_U32);
    pub const ISFORWARD: Self = Self(inner::BrowseResultMask::ISFORWARD_U32);
    pub const NODECLASS: Self = Self(inner::BrowseResultMask::NODECLASS_U32);
    pub const BROWSENAME: Self = Self(inner::BrowseResultMask::BROWSENAME_U32);
    pub const DISPLAYNAME: Self = Self(inner::BrowseResultMask::DISPLAYNAME_U32);
    pub const TYPEDEFINITION: Self = Self(inner::BrowseResultMask::TYPEDEFINITION_U32);
    pub const ALL: Self = Self(inner::BrowseResultMask::ALL_U32);
    pub const REFERENCETYPEINFO: Self = Self(inner::BrowseResultMask::REFERENCETYPEINFO_U32);
    pub const TARGETINFO: Self = Self(inner::BrowseResultMask::TARGETINFO_U32);

    #[deprecated(note = "use `Self::NONE` instead")]
    #[must_use]
    pub const fn none() -> Self {
        Self::NONE
    }

    #[deprecated(note = "use `Self::REFERENCETYPEID` instead")]
    #[must_use]
    pub const fn reference_type_id() -> Self {
        Self::REFERENCETYPEID
    }

    #[deprecated(note = "use `Self::ISFORWARD` instead")]
    #[must_use]
    pub const fn is_forward() -> Self {
        Self::ISFORWARD
    }

    #[deprecated(note = "use `Self::NODECLASS` instead")]
    #[must_use]
    pub const fn node_class() -> Self {
        Self::NODECLASS
    }

    #[deprecated(note = "use `Self::BROWSENAME` instead")]
    #[must_use]
    pub const fn browse_name() -> Self {
        Self::BROWSENAME
    }

    #[deprecated(note = "use `Self::DISPLAYNAME` instead")]
    #[must_use]
    pub const fn display_name() -> Self {
        Self::DISPLAYNAME
    }

    #[deprecated(note = "use `Self::TYPEDEFINITION` instead")]
    #[must_use]
    pub const fn type_definition() -> Self {
        Self::TYPEDEFINITION
    }

    #[deprecated(note = "use `Self::ALL` instead")]
    #[must_use]
    pub const fn all() -> Self {
        Self::ALL
    }

    #[deprecated(note = "use `Self::REFERENCETYPEINFO` instead")]
    #[must_use]
    pub const fn reference_type_info() -> Self {
        Self::REFERENCETYPEINFO
    }

    #[deprecated(note = "use `Self::TARGETINFO` instead")]
    #[must_use]
    pub const fn target_info() -> Self {
        Self::TARGETINFO
    }

    pub(crate) const fn from_u32(mask: u32) -> Self {
        Self(mask)
    }

    pub(crate) const fn as_u32(&self) -> u32 {
        self.0
    }
}
