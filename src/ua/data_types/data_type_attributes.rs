use crate::DataType;

crate::data_type!(DataTypeAttributes);


impl Default for DataTypeAttributes {
    fn default() -> Self {
        Self::clone_raw(unsafe { &open62541_sys::UA_DataTypeAttributes_default })
    }
}
