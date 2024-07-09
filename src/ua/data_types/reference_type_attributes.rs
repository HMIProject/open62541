use crate::DataType;

crate::data_type!(ReferenceTypeAttributes);

impl Default for ReferenceTypeAttributes {
    fn default() -> Self {
        Self::clone_raw(unsafe { &open62541_sys::UA_ReferenceTypeAttributes_default })
    }
}
