use open62541_sys::UA_BrowseDirection;

crate::data_type!(BrowseDirection);

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
}
