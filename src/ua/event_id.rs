use crate::ua;

/// Wrapper for event ID from [`open62541_sys`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventId(ua::ByteString);

impl EventId {
    /// Creates event ID from raw string.
    ///
    /// This may return [`None`] when the string is invalid (as defined by OPC UA).
    ///
    /// Note: The given string should not be empty.
    #[must_use]
    pub(crate) fn new(event_id: ua::ByteString) -> Option<Self> {
        // Unset event IDs are not expected.
        if event_id.is_invalid() {
            return None;
        }

        // An empty string would be a strange event ID. Nothing bad would happen since this is not
        // an invalid string (as defined by OPC UA) but it might indicate an error.
        debug_assert!(!event_id.is_empty());

        Some(Self(event_id))
    }

    /// Gets underlying representation.
    #[allow(dead_code)] // It is unclear whether external callers need the raw event ID.
    #[must_use]
    pub(crate) const fn as_byte_string(&self) -> &ua::ByteString {
        &self.0
    }

    /// Gets underlying representation.
    #[allow(dead_code)] // It is unclear whether external callers need the raw event ID.
    #[must_use]
    pub(crate) fn to_byte_string(&self) -> ua::ByteString {
        self.0.clone()
    }

    /// Gets underlying representation.
    #[allow(dead_code)] // It is unclear whether external callers need the raw event ID.
    #[must_use]
    pub(crate) fn into_byte_string(self) -> ua::ByteString {
        self.0
    }
}
