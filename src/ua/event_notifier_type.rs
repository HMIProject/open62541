use std::ops::{BitAnd, BitOr};

crate::data_type::data_type_baseline!(
    #[derive(Clone, Copy, Hash)]
    EventNotifierType
);

impl BitAnd for EventNotifierType {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl BitOr for EventNotifierType {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

macro_rules! impl_event_type {
    ($($event_t:ident),+ $(,)?) => {
        paste::paste! {
            impl EventNotifierType {
                $(
                    #[expect(clippy::as_conversion, reason = "well known variant must be representable as u8")]
                    pub const $event_t: Self = Self::from_u8(::open62541_sys::[<UA_EVENTNOTIFIERTYPE_ $event_t>] as u8);
                )+
            }


            #[cfg(test)]
            mod tests {
                $(
                    #[test]
                    fn [<event_notifier_type_ $event_t:lower _castable_to_u8>]() {
                        u8::try_from(::open62541_sys::[<UA_EVENTNOTIFIERTYPE_ $event_t>]).expect("EventNotifierType variant must be convertible to u8");
                    }
                )+
            }
        }
    }
}

impl_event_type! {
    NONE,
    SUBSCRIBETOEVENTS,
    HISTORYREAD,
    HISTORYWRITE,
}

// See <https://reference.opcfoundation.org/Core/Part3/v105/docs/8.59> for bit values.
impl EventNotifierType {
    pub(crate) const fn from_u8(mask: u8) -> Self {
        Self(mask)
    }

    #[must_use]
    pub const fn none(self) -> bool {
        self.0 == Self::NONE.0
    }

    /// Indicates if it can be used to subscribe to events.
    #[must_use]
    pub const fn subscribe_to_events(self) -> bool {
        self.0 & Self::SUBSCRIBETOEVENTS.0 != 0
    }

    /// Indicates if the history of the events is readable.
    #[must_use]
    pub const fn history_read(self) -> bool {
        self.0 & Self::HISTORYREAD.0 != 0
    }

    /// Indicates if the history of the events is writeable.
    #[must_use]
    pub const fn history_write(self) -> bool {
        self.0 & Self::HISTORYWRITE.0 != 0
    }
}

