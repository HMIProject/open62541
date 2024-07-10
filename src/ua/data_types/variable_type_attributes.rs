use crate::DataType;

crate::data_type!(VariableTypeAttributes);

impl VariableTypeAttributes {}

impl Default for VariableTypeAttributes {
    fn default() -> Self {
        Self::clone_raw(unsafe { &open62541_sys::UA_VariableTypeAttributes_default })
    }
}
