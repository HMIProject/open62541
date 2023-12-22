use crate::ua;

/// Wrapper for subscription ID from [`open62541_sys`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SubscriptionId(u32);

impl SubscriptionId {
    #[must_use]
    pub(crate) const fn new(id: u32) -> Self {
        Self(id)
    }

    pub(crate) fn as_u32(self) -> u32 {
        // This cast is necessary on Windows builds with inner type `i32`.
        #[allow(clippy::useless_conversion)]
        u32::from(self.0)
    }

    pub(crate) fn to_uint32(self) -> ua::UInt32 {
        ua::UInt32::new(self.as_u32())
    }
}
