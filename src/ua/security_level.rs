use open62541_sys::UA_Byte;

use crate::ua;

/// Wrapper for security level from [`open62541_sys`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SecurityLevel(UA_Byte);

impl SecurityLevel {
    pub(crate) const fn new(security_level: UA_Byte) -> Self {
        Self(security_level)
    }

    #[allow(dead_code)] // This is unused for now.
    pub(crate) const fn as_u8(self) -> u8 {
        self.0
    }

    #[allow(dead_code)] // This is unused for now.
    pub(crate) const fn to_byte(self) -> ua::Byte {
        ua::Byte::new(self.as_u8())
    }
}
