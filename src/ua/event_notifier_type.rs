use open62541_sys::UA_EventNotifierType;

use crate::{ua, DataTypeExt};

/// Wrapper for [`UA_EventNotifierType`] from [`open62541_sys`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EventNotifierType(UA_EventNotifierType);

// See <https://reference.opcfoundation.org/Core/Part3/v105/docs/8.59> for bit values.
impl EventNotifierType {
    pub(crate) const fn from_u8(mask: u8) -> Self {
        Self(mask)
    }

    pub(crate) const fn as_u8(&self) -> u8 {
        self.0
    }

    /// Indicates if it can be used to subscribe to events.
    pub const fn subscribe_to_events(&self) -> bool {
        self.0 & (1 << 0) != 0
    }

    /// Indicates if the history of the events is readable.
    pub const fn history_read(&self) -> bool {
        self.0 & (1 << 2) != 0
    }

    /// Indicates if the history of the events is writeable.
    pub const fn history_write(&self) -> bool {
        self.0 & (1 << 3) != 0
    }
}

impl DataTypeExt for EventNotifierType {
    type Inner = ua::Byte;

    fn from_inner(value: Self::Inner) -> Self {
        Self::from_u8(value.value())
    }

    fn into_inner(self) -> Self::Inner {
        Self::Inner::new(self.as_u8())
    }
}
