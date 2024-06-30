use open62541_sys::{UA_ACCESSLEVELTYPE_CURRENTREAD, UA_ACCESSLEVELTYPE_CURRENTWRITE};

/// Wrapper for access level from [`open62541_sys`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AccessLevel(u8);

impl AccessLevel {
    pub const NONE: Self = Self(0);

    #[must_use]
    pub fn with_current_read(self, current_read: bool) -> Self {
        self.apply_mask(UA_ACCESSLEVELTYPE_CURRENTREAD, current_read)
    }

    #[must_use]
    pub fn with_current_write(self, current_write: bool) -> Self {
        self.apply_mask(UA_ACCESSLEVELTYPE_CURRENTWRITE, current_write)
    }

    fn apply_mask(mut self, mask: u32, flag: bool) -> Self {
        // PANIC: Mask is always in range of `u8`.
        let mask = u8::try_from(mask).unwrap_or(0);
        if flag {
            self.0 |= mask;
        } else {
            self.0 &= !mask;
        }
        self
    }

    pub(crate) const fn as_u8(&self) -> u8 {
        self.0
    }
}
