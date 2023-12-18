/// Wrapper for node class mask from [`open62541_sys`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeClassMask(u32);

impl NodeClassMask {
    /// Creates wrapper by taking ownership of `src`.
    #[allow(dead_code)]
    #[must_use]
    pub(crate) const fn new(src: u32) -> Self {
        Self(src)
    }

    /// Gives up ownership and returns inner value.
    #[must_use]
    pub(crate) const fn into_inner(self) -> u32 {
        self.0
    }
}
