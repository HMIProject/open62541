use open62541_sys::UA_ServerStatistics;

use crate::ua;

#[derive(Debug)]
pub struct ServerStatistics(UA_ServerStatistics);

impl ServerStatistics {
    /// Creates wrapper by taking ownership of value.
    ///
    /// When `Self` is dropped, allocations held by the inner type are cleaned up.
    ///
    /// # Safety
    ///
    /// Ownership of the value passes to `Self`. This must only be used for values that are not
    /// contained within other values that may be dropped (such as attributes in other data types).
    #[must_use]
    pub(crate) const unsafe fn from_raw(src: UA_ServerStatistics) -> Self {
        Self(src)
    }

    #[must_use]
    pub const fn scs(&self) -> &ua::SecureChannelStatistics {
        ua::SecureChannelStatistics::raw_ref(&self.0.scs)
    }

    #[must_use]
    pub const fn ss(&self) -> &ua::SessionStatistics {
        ua::SessionStatistics::raw_ref(&self.0.ss)
    }
}
