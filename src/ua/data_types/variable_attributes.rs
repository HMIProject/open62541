use open62541_sys::UA_NodeAttributes;

use crate::{ua, DataType as _};

crate::data_type!(VariableAttributes);

impl VariableAttributes {
    #[must_use]
    pub fn with_data_type(mut self, data_type: &ua::NodeId) -> Self {
        data_type.clone_into_raw(&mut self.0.dataType);
        self
    }

    #[must_use]
    pub fn with_access_level(mut self, access_level: &ua::AccessLevel) -> Self {
        self.0.accessLevel = access_level.as_u8();
        self
    }

    pub(crate) fn as_node_attributes(&self) -> &ua::NodeAttributes {
        // SAFETY: This transmutes from `Self` to the inner type, and then to `UA_NodeAttributes`, a
        // subset of `UA_VariableAttributes` with the same memory layout.
        let node_attributes = unsafe { self.as_ptr().cast::<UA_NodeAttributes>() };
        // SAFETY: Transmutation is allowed and pointer is valid (non-zero).
        let node_attributes = unsafe { node_attributes.as_ref().unwrap_unchecked() };
        ua::NodeAttributes::raw_ref(node_attributes)
    }
}

impl Default for VariableAttributes {
    fn default() -> Self {
        Self::clone_raw(unsafe { &open62541_sys::UA_VariableAttributes_default })
    }
}
