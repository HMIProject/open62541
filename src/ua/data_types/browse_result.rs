use crate::{ua, DataType as _};

crate::data_type!(BrowseResult);

impl BrowseResult {
    #[must_use]
    pub fn references(&self) -> Option<ua::Array<ua::ReferenceDescription>> {
        // TODO: Adjust signature to return non-owned value instead.
        ua::Array::from_raw_parts(self.0.references, self.0.referencesSize)
    }
}
