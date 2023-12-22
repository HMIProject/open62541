use open62541_sys::{UA_StatusCode, UA_STATUSCODE_GOOD};

crate::data_type!(StatusCode);

impl StatusCode {
    /// Creates wrapper by taking ownership of `src`.
    #[must_use]
    pub(crate) const fn new(src: UA_StatusCode) -> Self {
        Self(src)
    }

    /// Checks if status code is good.
    #[must_use]
    pub(crate) const fn is_good(&self) -> bool {
        // TODO: Check name of this method. Consider potential clash with `UA_StatusCode_isGood()`
        // which only makes check for _severity_ of status code (i.e. may match an entire range of
        // codes).
        self.0 == UA_STATUSCODE_GOOD
    }
}
