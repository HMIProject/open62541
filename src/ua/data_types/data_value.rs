use crate::ua;

crate::data_type!(DataValue, UA_DataValue, UA_TYPES_DATAVALUE);

impl DataValue {
    #[must_use]
    pub fn value(&self) -> Option<ua::Variant> {
        // TODO: Adjust signature to return non-owned value instead.
        if self.0.hasValue() {
            Some(ua::Variant::from_ref(&self.0.value))
        } else {
            None
        }
    }
}
