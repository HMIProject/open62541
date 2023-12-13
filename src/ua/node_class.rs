/// Wrapper for node classes from [`open62541_sys`].
#[derive(Clone, Copy, Debug)]
pub struct NodeClass(u32);

impl NodeClass {
    /// Creates wrapper by taking ownership of `src`.
    #[must_use]
    pub(crate) const fn new(src: u32) -> Self {
        Self(src)
    }

    /// Gives up ownership and returns inner value.
    #[allow(dead_code)]
    #[must_use]
    pub(crate) const fn into_inner(self) -> u32 {
        self.0
    }
}
