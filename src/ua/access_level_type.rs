use open62541_sys::{
    UA_ACCESSLEVELTYPE_CURRENTREAD, UA_ACCESSLEVELTYPE_CURRENTWRITE,
    UA_ACCESSLEVELTYPE_HISTORYREAD, UA_ACCESSLEVELTYPE_HISTORYWRITE,
    UA_ACCESSLEVELTYPE_SEMANTICCHANGE, UA_ACCESSLEVELTYPE_STATUSWRITE,
    UA_ACCESSLEVELTYPE_TIMESTAMPWRITE, UA_AccessLevelType,
};

use crate::{DataTypeExt, ua};

/// Wrapper for [`UA_AccessLevelType`] from [`open62541_sys`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AccessLevelType(UA_AccessLevelType);

// See <https://reference.opcfoundation.org/Core/Part3/v104/docs/8.57> for bit values.
impl AccessLevelType {
    pub const NONE: Self = Self(0);

    pub(crate) const fn from_u8(mask: u8) -> Self {
        Self(mask)
    }

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

    /// Indicates if the current value is readable.
    ///
    /// It also indicates if the current value of the variable is available.
    #[must_use]
    pub fn current_read(&self) -> bool {
        u32::from(self.0) & UA_ACCESSLEVELTYPE_CURRENTREAD != 0
    }

    /// Indicates if the current value is writable.
    ///
    /// It also indicates if the current value of the variable is available.
    #[must_use]
    pub fn current_write(&self) -> bool {
        u32::from(self.0) & UA_ACCESSLEVELTYPE_CURRENTWRITE != 0
    }

    /// Indicates if the history of the value is readable.
    ///
    /// It also indicates if the history of the variable is available via the OPC UA Server.
    #[must_use]
    pub fn history_read(&self) -> bool {
        u32::from(self.0) & UA_ACCESSLEVELTYPE_HISTORYREAD != 0
    }

    /// Indicates if the history of the value is writable.
    ///
    /// It also indicates if the history of the variable is available via the OPC UA Server.
    #[must_use]
    pub fn history_write(&self) -> bool {
        u32::from(self.0) & UA_ACCESSLEVELTYPE_HISTORYWRITE != 0
    }

    /// This flag is set for properties that define semantic aspects of the parent node of the
    /// property and where the property value, and thus the semantic, may change during operation.
    #[must_use]
    pub fn semantic_change(&self) -> bool {
        u32::from(self.0) & UA_ACCESSLEVELTYPE_SEMANTICCHANGE != 0
    }

    /// Indicates if the current status code of the value is writable.
    #[must_use]
    pub fn status_write(&self) -> bool {
        u32::from(self.0) & UA_ACCESSLEVELTYPE_STATUSWRITE != 0
    }

    /// Indicates if the current source timestamp is writable.
    #[must_use]
    pub fn timestamp_write(&self) -> bool {
        u32::from(self.0) & UA_ACCESSLEVELTYPE_TIMESTAMPWRITE != 0
    }
}

impl DataTypeExt for AccessLevelType {
    type Inner = ua::Byte;

    fn from_inner(value: Self::Inner) -> Self {
        Self::from_u8(value.value())
    }

    fn into_inner(self) -> Self::Inner {
        Self::Inner::new(self.as_u8())
    }
}

#[deprecated = "Use AccessLevelType instead."]
pub type AccessLevel = AccessLevelType;
