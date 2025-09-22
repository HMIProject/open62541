use open62541_sys::{
    UA_EVENTNOTIFIERTYPE_HISTORYREAD, UA_EVENTNOTIFIERTYPE_HISTORYWRITE,
    UA_EVENTNOTIFIERTYPE_SUBSCRIBETOEVENTS, UA_EventNotifierType,
};

use crate::{DataTypeExt, ua};

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
    #[must_use]
    pub fn subscribe_to_events(&self) -> bool {
        u32::from(self.0) & UA_EVENTNOTIFIERTYPE_SUBSCRIBETOEVENTS != 0
    }

    /// Indicates if the history of the events is readable.
    #[must_use]
    pub fn history_read(&self) -> bool {
        u32::from(self.0) & UA_EVENTNOTIFIERTYPE_HISTORYREAD != 0
    }

    /// Indicates if the history of the events is writeable.
    #[must_use]
    pub fn history_write(&self) -> bool {
        u32::from(self.0) & UA_EVENTNOTIFIERTYPE_HISTORYWRITE != 0
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
