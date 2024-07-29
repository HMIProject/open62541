use crate::ua;

/// Wrapper for continuation point from [`open62541_sys`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContinuationPoint(ua::ByteString);

impl ContinuationPoint {
    /// Creates continuation point from raw string.
    ///
    /// This may return [`None`] when the string is invalid (as defined by OPC UA). This is used in
    /// [`ua::BrowseResult`] to indicate that no continuation point was necessary.
    ///
    /// Note: The given string should not be empty.
    #[must_use]
    pub(crate) fn new(continuation_point: ua::ByteString) -> Option<Self> {
        // Unset continuation points indicate that the `BrowseResult` contains all references and no
        // continuation is actually necessary.
        if continuation_point.is_invalid() {
            return None;
        }

        // An empty string would be a strange continuation point. Nothing bad would happen since
        // this is not an invalid string (as defined by OPC UA) but it might indicate an error.
        debug_assert!(!continuation_point.is_empty());

        Some(Self(continuation_point))
    }

    /// Gets underlying representation.
    pub(crate) fn to_byte_string(&self) -> ua::ByteString {
        self.0.clone()
    }
}
