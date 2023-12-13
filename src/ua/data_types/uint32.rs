crate::data_type!(Uint32, UA_UInt32, UA_TYPES_UINT32);

impl Uint32 {
    #[must_use]
    pub const fn value(value: u32) -> Self {
        Self::new(value)
    }
}
