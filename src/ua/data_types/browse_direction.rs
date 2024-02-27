use open62541_sys::UA_BrowseDirection;

crate::data_type!(BrowseDirection);

crate::enum_variants!(
    BrowseDirection,
    UA_BrowseDirection,
    [FORWARD, INVERSE, BOTH, INVALID],
);

impl BrowseDirection {
    #[deprecated(note = "use `Self::FORWARD` instead")]
    #[must_use]
    pub const fn forward() -> Self {
        Self(UA_BrowseDirection::UA_BROWSEDIRECTION_FORWARD)
    }

    #[deprecated(note = "use `Self::INVERSE` instead")]
    #[must_use]
    pub const fn inverse() -> Self {
        Self(UA_BrowseDirection::UA_BROWSEDIRECTION_INVERSE)
    }

    #[deprecated(note = "use `Self::BOTH` instead")]
    #[must_use]
    pub const fn both() -> Self {
        Self(UA_BrowseDirection::UA_BROWSEDIRECTION_BOTH)
    }

    #[deprecated(note = "use `Self::INVALID` instead")]
    #[must_use]
    pub const fn invalid() -> Self {
        Self(UA_BrowseDirection::UA_BROWSEDIRECTION_INVALID)
    }
}
