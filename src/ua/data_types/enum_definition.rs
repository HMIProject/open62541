use crate::ua;

crate::data_type!(EnumDefinition);

impl EnumDefinition {
    #[must_use]
    pub fn fields(&self) -> Option<ua::Array<ua::EnumField>> {
        // TODO: Adjust signature to return non-owned value instead.
        ua::Array::from_raw_parts(self.0.fieldsSize, self.0.fields)
    }
}
