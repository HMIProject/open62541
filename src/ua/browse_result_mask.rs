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

    pub(crate) const fn from_u32(mask: u32) -> Self {
        Self(mask)
    }

    pub(crate) const fn as_u32(&self) -> u32 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::ua;

    #[test]
    fn combine_masks() {
        // We support mask combinators with `|` shorthand notation.
        let lhs = ua::BrowseResultMask::REFERENCETYPEID | ua::BrowseResultMask::ISFORWARD;
        let rhs = ua::BrowseResultMask::REFERENCETYPEINFO;
        assert_eq!(lhs, rhs);

        // We support mask combinators in `const` expressions.
        const LHS: ua::BrowseResultMask =
            ua::BrowseResultMask::REFERENCETYPEID.or(&ua::BrowseResultMask::ISFORWARD);
        const RHS: ua::BrowseResultMask = ua::BrowseResultMask::REFERENCETYPEINFO;
        assert_eq!(LHS, RHS);
    }
}
