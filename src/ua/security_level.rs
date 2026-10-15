//! Wrapper for security level from [`open62541_sys`].
use open62541_sys::UA_Byte;

use crate::ua;

crate::data_type::data_type_baseline! {
    #[expect(
        clippy::derived_hash_with_manual_eq,
        reason = "open62541 numeric equality impl satisfies `Hash`s identity invariant"
    )]
    #[derive(Clone, Copy, Hash)]
    SecurityLevel,
    #[uses_inner]
    Byte
}

impl SecurityLevel {
    #[must_use]
    pub const fn new(security_level: UA_Byte) -> Self {
        Self(security_level)
    }

    #[must_use]
    pub(crate) const fn as_u8(self) -> u8 {
        self.0
    }

    #[expect(dead_code, reason = "unused for now")]
    pub(crate) const fn to_byte(self) -> ua::Byte {
        ua::Byte::new(self.as_u8())
    }
}
