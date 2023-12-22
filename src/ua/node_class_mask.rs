/// Wrapper for node class mask from [`open62541_sys`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeClassMask(u32);

impl NodeClassMask {
    pub(crate) const fn as_u32(&self) -> u32 {
        self.0
    }
}
