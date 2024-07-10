use crate::DataType;

crate::data_type!(ObjectAttributes);

impl Default for ObjectAttributes {
    fn default() -> Self {
        Self::clone_raw(unsafe { &open62541_sys::UA_ObjectAttributes_default })
    }
}
