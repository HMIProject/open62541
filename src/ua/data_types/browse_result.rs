use crate::{ua, DataType as _};

crate::data_type!(BrowseResult);

impl BrowseResult {
    #[must_use]
    pub fn references(&self) -> Option<ua::Array<ua::ReferenceDescription>> {
        // TODO: Adjust signature to return non-owned value instead.
        ua::Array::from_raw_parts(self.0.references, self.0.referencesSize)
    }

    /// Gets continuation point.
    ///
    /// Browse results include a continuation point when not all references could be returned. Pass
    /// it to [`AsyncClient::browse_next()`] to request the remaining references.
    ///
    /// [`AsyncClient::browse_next()`]: crate::AsyncClient::browse_next
    #[must_use]
    pub fn continuation_point(&self) -> Option<ua::ContinuationPoint> {
        ua::ContinuationPoint::new(ua::ByteString::raw_ref(&self.0.continuationPoint))
    }
}
