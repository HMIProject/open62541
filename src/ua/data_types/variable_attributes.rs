use crate::DataType as _;

crate::data_type!(VariableAttributes);

impl Default for VariableAttributes {
    fn default() -> Self {
        Self::clone_raw(unsafe { &open62541_sys::UA_VariableAttributes_default })
    }
}
