/// Wrapper for node class mask from [`open62541_sys`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeClassMask(u32);

impl NodeClassMask {
    pub(crate) fn as_u32(&self) -> u32 {
        // This cast is necessary on Windows builds with inner type `i32`.
        #[allow(clippy::useless_conversion)]
        u32::from(self.0)
    }
}
