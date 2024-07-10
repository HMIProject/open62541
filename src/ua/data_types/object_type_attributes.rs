use crate::DataType;

crate::data_type!(ObjectTypeAttributes);

impl Default for ObjectTypeAttributes {
    fn default() -> Self {
        Self::clone_raw(unsafe { &open62541_sys::UA_ObjectTypeAttributes_default })
    }
}
