use open62541_sys::UA_NodeClass;

/// Wrapper for node classes from [`open62541_sys`].
#[derive(Clone, Debug)]
pub struct NodeClass(UA_NodeClass);

impl NodeClass {
    /// Creates wrapper by taking ownership of `src`.
    #[must_use]
    pub(crate) const fn new(src: UA_NodeClass) -> Self {
        Self(src)
    }

    /// Gives up ownership and returns inner value.
    #[allow(dead_code)]
    #[must_use]
    pub(crate) const fn into_inner(self) -> UA_NodeClass {
        self.0
    }
}
