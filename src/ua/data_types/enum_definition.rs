use crate::ua;

use super::EnumField;

crate::data_type!(EnumDefinition);

impl EnumDefinition {
    #[must_use]
    pub fn fields(&self) -> Option<ua::Array<EnumField>> {
        // TODO: Adjust signature to return non-owned value instead.
        ua::Array::from_raw_parts(self.0.fieldsSize, self.0.fields)
    }
}
