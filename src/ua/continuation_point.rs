use crate::ua;

/// Wrapper for continuation point from [`open62541_sys`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContinuationPoint(ua::ByteString);

impl ContinuationPoint {
    #[must_use]
    pub(crate) fn new(continuation_point: &ua::ByteString) -> Option<Self> {
        // Unset continuation points indicate that the `BrowseResult` contains all references and no
        // continuation is actually necessary.
        if continuation_point.is_invalid() {
            return None;
        }

        // The continuation point should not be an empty string.
        debug_assert!(!continuation_point.is_empty());

        Some(Self(continuation_point.clone()))
    }

    pub(crate) fn to_byte_string(&self) -> ua::ByteString {
        self.0.clone()
    }
}
