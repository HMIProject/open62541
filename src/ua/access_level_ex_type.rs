use open62541_sys::{
    UA_ACCESSLEVELEXTYPE_CONSTANT, UA_ACCESSLEVELEXTYPE_CURRENTREAD,
    UA_ACCESSLEVELEXTYPE_CURRENTWRITE, UA_ACCESSLEVELEXTYPE_HISTORYREAD,
    UA_ACCESSLEVELEXTYPE_HISTORYWRITE, UA_ACCESSLEVELEXTYPE_NONATOMICREAD,
    UA_ACCESSLEVELEXTYPE_NONATOMICWRITE, UA_ACCESSLEVELEXTYPE_NONVOLATILE,
    UA_ACCESSLEVELEXTYPE_NOSUBDATATYPES, UA_ACCESSLEVELEXTYPE_SEMANTICCHANGE,
    UA_ACCESSLEVELEXTYPE_STATUSWRITE, UA_ACCESSLEVELEXTYPE_TIMESTAMPWRITE,
    UA_ACCESSLEVELEXTYPE_WRITEFULLARRAYONLY, UA_AccessLevelExType,
};

use crate::{DataTypeExt, ua};

/// Wrapper for [`UA_AccessLevelExType`] from [`open62541_sys`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AccessLevelExType(UA_AccessLevelExType);

// See <https://reference.opcfoundation.org/Core/Part3/v105/docs/8.58> for bit values.
impl AccessLevelExType {
    pub(crate) const fn from_u32(mask: u32) -> Self {
        Self(mask)
    }

    pub(crate) const fn as_u32(&self) -> u32 {
        self.0
    }

    /// Indicates if the current value is readable.
    ///
    /// It also indicates if the current value of the variable is available.
    #[must_use]
    pub const fn current_read(&self) -> bool {
        self.0 & UA_ACCESSLEVELEXTYPE_CURRENTREAD != 0
    }

    /// Indicates if the current value is writable.
    ///
    /// It also indicates if the current value of the variable is available.
    #[must_use]
    pub const fn current_write(&self) -> bool {
        self.0 & UA_ACCESSLEVELEXTYPE_CURRENTWRITE != 0
    }

    /// Indicates if the history of the value is readable.
    ///
    /// It also indicates if the history of the variable is available via the OPC UA Server.
    #[must_use]
    pub const fn history_read(&self) -> bool {
        self.0 & UA_ACCESSLEVELEXTYPE_HISTORYREAD != 0
    }

    /// Indicates if the history of the value is writable.
    ///
    /// It also indicates if the history of the variable is available via the OPC UA Server.
    #[must_use]
    pub const fn history_write(&self) -> bool {
        self.0 & UA_ACCESSLEVELEXTYPE_HISTORYWRITE != 0
    }

    /// This flag is set for properties that define semantic aspects of the parent node of the
    /// property and where the property value, and thus the semantic, may change during operation.
    #[must_use]
    pub const fn semantic_change(&self) -> bool {
        self.0 & UA_ACCESSLEVELEXTYPE_SEMANTICCHANGE != 0
    }

    /// Indicates if the current status code of the value is writable.
    #[must_use]
    pub const fn status_write(&self) -> bool {
        self.0 & UA_ACCESSLEVELEXTYPE_STATUSWRITE != 0
    }

    /// Indicates if the current source timestamp is writable.
    #[must_use]
    pub const fn timestamp_write(&self) -> bool {
        self.0 & UA_ACCESSLEVELEXTYPE_TIMESTAMPWRITE != 0
    }

    /// Indicates non-atomicity for read access.
    #[must_use]
    pub const fn nonatomic_read(&self) -> bool {
        self.0 & UA_ACCESSLEVELEXTYPE_NONATOMICREAD != 0
    }

    /// Indicates non-atomicity for write access.
    #[must_use]
    pub const fn nonatomic_write(&self) -> bool {
        self.0 & UA_ACCESSLEVELEXTYPE_NONATOMICWRITE != 0
    }

    /// Indicates if write of index range is supported.
    #[must_use]
    pub const fn write_full_array_only(&self) -> bool {
        self.0 & UA_ACCESSLEVELEXTYPE_WRITEFULLARRAYONLY != 0
    }

    /// Indicates if the variable doesn’t allow its data type to be subtyped.
    #[must_use]
    pub const fn no_sub_data_types(&self) -> bool {
        self.0 & UA_ACCESSLEVELEXTYPE_NOSUBDATATYPES != 0
    }

    /// Indicates if the variable is non-volatile.
    #[must_use]
    pub const fn non_volatile(&self) -> bool {
        self.0 & UA_ACCESSLEVELEXTYPE_NONVOLATILE != 0
    }

    /// Indicates if the value of the variable can be considered constant.
    #[must_use]
    pub const fn constant(&self) -> bool {
        self.0 & UA_ACCESSLEVELEXTYPE_CONSTANT != 0
    }
}

impl DataTypeExt for AccessLevelExType {
    type Inner = ua::UInt32;

    fn from_inner(value: Self::Inner) -> Self {
        Self::from_u32(value.value())
    }

    fn into_inner(self) -> Self::Inner {
        Self::Inner::new(self.as_u32())
    }
}
