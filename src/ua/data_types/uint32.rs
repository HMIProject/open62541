crate::data_type!(Uint32, UA_UInt32, UA_TYPES_UINT32);

impl Uint32 {
    #[must_use]
    pub fn new(value: u32) -> Self {
        Self::from_ref(&value)
    }
}
