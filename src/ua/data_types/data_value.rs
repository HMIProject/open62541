use crate::ua;

crate::data_type!(DataValue, UA_DataValue, UA_TYPES_DATAVALUE);

impl DataValue {
    #[must_use]
    pub fn value(&self) -> ua::Variant {
        // TODO: Adjust signature to return non-owned value instead.
        ua::Variant::from_ref(&self.0.value)
    }
}
