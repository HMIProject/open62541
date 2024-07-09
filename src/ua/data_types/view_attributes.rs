use crate::DataType;

crate::data_type!(ViewAttributes);

impl Default for ViewAttributes {
    fn default() -> Self {
        Self::clone_raw(unsafe { &open62541_sys::UA_ViewAttributes_default })
    }
}
