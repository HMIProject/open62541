use open62541_sys::UA_AttributeId;

crate::data_type!(AttributeId, UInt32);

impl AttributeId {
    #[must_use]
    pub const fn value() -> Self {
        Self(UA_AttributeId::UA_ATTRIBUTEID_VALUE)
    }

    pub(crate) fn as_u32(&self) -> u32 {
        // This cast is necessary on Windows builds with inner type `i32`.
        #[allow(clippy::useless_conversion)]
        u32::try_from((self.0).0).expect("should convert to u32")
    }
}
