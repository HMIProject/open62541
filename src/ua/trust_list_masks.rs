#[expect(unreachable_pub, reason = "hidden inner enum")]
// We do not expose the inner enum. We want to use a proper `u32` for bit operations on the mask and
// we want to be clear about what is an initial (const, enum-like) value and what is a derived mask;
// specifically, the bitmask type is _not_ an enum even though declared so in `open62541-sys`.
mod inner {
    crate::data_type!(TrustListMasks);

    crate::enum_variants!(
        TrustListMasks,
        UA_TrustListMasks,
        [
            NONE,
            TRUSTEDCERTIFICATES,
            TRUSTEDCRLS,
            ISSUERCERTIFICATES,
            ISSUERCRLS,
            ALL,
        ],
    );
}

/// Wrapper for trust list masks from [`open62541_sys`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TrustListMasks(u32);

crate::bitmask_ops!(TrustListMasks);

impl TrustListMasks {
    pub const NONE: Self = Self(inner::TrustListMasks::NONE_U32);
    pub const TRUSTEDCERTIFICATES: Self = Self(inner::TrustListMasks::TRUSTEDCERTIFICATES_U32);
    pub const TRUSTEDCRLS: Self = Self(inner::TrustListMasks::TRUSTEDCRLS_U32);
    pub const ISSUERCERTIFICATES: Self = Self(inner::TrustListMasks::ISSUERCERTIFICATES_U32);
    pub const ISSUERCRLS: Self = Self(inner::TrustListMasks::ISSUERCRLS_U32);
    pub const ALL: Self = Self(inner::TrustListMasks::ALL_U32);

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
        // We support mask combinators in `const` expressions.
        const LHS: ua::TrustListMasks = ua::TrustListMasks::TRUSTEDCERTIFICATES
            .or(&ua::TrustListMasks::TRUSTEDCRLS)
            .or(&ua::TrustListMasks::ISSUERCERTIFICATES)
            .or(&ua::TrustListMasks::ISSUERCRLS);
        const RHS: ua::TrustListMasks = ua::TrustListMasks::ALL;
        assert_eq!(LHS, RHS);

        // We support mask combinators with `|` shorthand notation.
        let lhs = ua::TrustListMasks::TRUSTEDCERTIFICATES
            | ua::TrustListMasks::TRUSTEDCRLS
            | ua::TrustListMasks::ISSUERCERTIFICATES
            | ua::TrustListMasks::ISSUERCRLS;
        let rhs = ua::TrustListMasks::ALL;
        assert_eq!(lhs, rhs);
    }
}
