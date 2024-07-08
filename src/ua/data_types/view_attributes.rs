use open62541_sys::UA_NodeAttributes;

use crate::{ua, DataType};

crate::data_type!(ViewAttributes);

impl ViewAttributes {
    pub(crate) fn as_node_attributes(&self) -> &ua::NodeAttributes {
        // SAFETY: This transmutes from `Self` to the inner type, and then to `UA_NodeAttributes`, a
        // subset of `UA_ViewAttributes` with the same memory layout.
        let node_attributes = unsafe { self.as_ptr().cast::<UA_NodeAttributes>() };
        // SAFETY: Transmutation is allowed and pointer is valid (non-zero).
        let node_attributes = unsafe { node_attributes.as_ref().unwrap_unchecked() };
        ua::NodeAttributes::raw_ref(node_attributes)
    }
}

impl Default for ViewAttributes {
    fn default() -> Self {
        Self::clone_raw(unsafe { &open62541_sys::UA_ViewAttributes_default })
    }
}
