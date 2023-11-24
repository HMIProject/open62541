use std::ptr::{self, NonNull};

use open62541_sys::{
    UA_NodeId_copy, UA_ReadValueId, UA_ReadValueId_delete, UA_ReadValueId_new, UA_STATUSCODE_GOOD,
    UA_TYPES_READVALUEID,
};

use crate::ua;

pub struct ReadValueId(NonNull<UA_ReadValueId>);

impl ReadValueId {
    #[must_use]
    pub fn new() -> Option<Self> {
        // `UA_ReadValueId_new` matches `UA_ReadValueId_delete`.
        let ua_read_value_id = NonNull::new(unsafe { UA_ReadValueId_new() })?;

        Some(Self(ua_read_value_id))
    }

    #[must_use]
    pub fn node_id(mut self, node_id: &ua::NodeId) -> Option<Self> {
        let src = node_id.as_ptr();
        let dst = ptr::addr_of_mut!(unsafe { self.0.as_mut() }.nodeId);

        let result = unsafe { UA_NodeId_copy(src, dst) };

        if result != UA_STATUSCODE_GOOD {
            return None;
        }

        Some(self)
    }

    #[must_use]
    pub fn attribute_id(mut self, attribute_id: u32) -> Option<Self> {
        unsafe { self.0.as_mut() }.attributeId = attribute_id;

        Some(self)
    }

    #[must_use]
    #[allow(dead_code)]
    pub(crate) const fn as_ptr(&self) -> *const UA_ReadValueId {
        self.0.as_ptr()
    }

    #[must_use]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut UA_ReadValueId {
        self.0.as_ptr()
    }
}

impl Drop for ReadValueId {
    fn drop(&mut self) {
        // `UA_ReadValueId_delete` matches `UA_ReadValueId_new`.
        unsafe { UA_ReadValueId_delete(self.as_mut_ptr()) }
    }
}

impl ua::DataType for ReadValueId {
    type Inner = UA_ReadValueId;

    const INNER: u32 = UA_TYPES_READVALUEID;

    fn as_ptr(&self) -> *const Self::Inner {
        ReadValueId::as_ptr(self)
    }
}
