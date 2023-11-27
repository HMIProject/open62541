use crate::ua;

ua::data_type!(DataValue, UA_DataValue, UA_TYPES_DATAVALUE);

impl DataValue {
    #[must_use]
    pub fn value(&self) -> ua::Variant {
        ua::Variant::from(&self.0.value)
    }
}
