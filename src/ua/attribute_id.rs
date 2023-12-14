use open62541_sys::UA_AttributeId;

/// Wrapper for attribute IDs from [`open62541_sys`].
#[derive(Clone, Debug)]
pub struct AttributeId(UA_AttributeId);

impl AttributeId {
    #[must_use]
    pub const fn value() -> Self {
        Self(UA_AttributeId::UA_ATTRIBUTEID_VALUE)
    }

    /// Creates wrapper by taking ownership of `src`.
    #[allow(dead_code)]
    #[must_use]
    pub(crate) const fn new(src: UA_AttributeId) -> Self {
        Self(src)
    }

    /// Gives up ownership and returns inner value.
    #[must_use]
    pub(crate) const fn into_inner(self) -> UA_AttributeId {
        self.0
    }
}
