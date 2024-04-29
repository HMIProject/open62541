crate::data_type!(TimestampsToReturn);

crate::enum_variants!(
    TimestampsToReturn,
    UA_TimestampsToReturn,
    [SOURCE, SERVER, BOTH, NEITHER, INVALID],
);

impl TimestampsToReturn {
    #[deprecated(note = "use `Self::SOURCE` instead")]
    #[must_use]
    pub const fn source() -> Self {
        Self::SOURCE
    }

    #[deprecated(note = "use `Self::SERVER` instead")]
    #[must_use]
    pub const fn server() -> Self {
        Self::SERVER
    }

    #[deprecated(note = "use `Self::BOTH` instead")]
    #[must_use]
    pub const fn both() -> Self {
        Self::BOTH
    }

    #[deprecated(note = "use `Self::NEITHER` instead")]
    #[must_use]
    pub const fn neither() -> Self {
        Self::NEITHER
    }

    #[deprecated(note = "use `Self::INVALID` instead")]
    #[must_use]
    pub const fn invalid() -> Self {
        Self::INVALID
    }
}
