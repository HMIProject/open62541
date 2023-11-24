use std::{mem, ptr};

use open62541_sys::{
    UA_NodeId_clear, UA_ReadValueId, UA_ReadValueId_clear, UA_ReadValueId_copy,
    UA_ReadValueId_init, UA_STATUSCODE_GOOD, UA_TYPES_READVALUEID,
};

use crate::ua;

pub struct ReadValueId(UA_ReadValueId);

impl ReadValueId {
    #[must_use]
    pub fn new() -> Self {
        let mut inner = unsafe { mem::MaybeUninit::<UA_ReadValueId>::zeroed().assume_init() };
        unsafe { UA_ReadValueId_init(ptr::addr_of_mut!(inner)) }
        Self(inner)
    }

    /// Copies value from `src`.
    #[allow(dead_code)]
    pub(crate) fn from(src: &UA_ReadValueId) -> Self {
        let mut dst = Self::new();

        let result = unsafe { UA_ReadValueId_copy(src, dst.as_mut_ptr()) };
        assert_eq!(result, UA_STATUSCODE_GOOD);

        dst
    }

    /// Takes ownership of `src`.
    #[allow(dead_code)]
    pub(crate) fn from_inner(src: UA_ReadValueId) -> Self {
        Self(src)
    }

    #[allow(dead_code)]
    pub(crate) const fn as_ref(&self) -> &UA_ReadValueId {
        &self.0
    }

    #[allow(dead_code)]
    pub(crate) fn as_mut(&mut self) -> &mut UA_ReadValueId {
        &mut self.0
    }

    #[allow(dead_code)]
    pub(crate) const fn as_ptr(&self) -> *const UA_ReadValueId {
        ptr::addr_of!(self.0)
    }

    #[allow(dead_code)]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut UA_ReadValueId {
        ptr::addr_of_mut!(self.0)
    }

    #[allow(dead_code)]
    pub(crate) fn into_inner(self) -> UA_ReadValueId {
        let inner = self.0;
        mem::forget(self);
        inner
    }
}

impl Drop for ReadValueId {
    fn drop(&mut self) {
        unsafe { UA_ReadValueId_clear(self.as_mut_ptr()) }
    }
}

impl Default for ReadValueId {
    fn default() -> Self {
        Self::new()
    }
}

impl ua::DataType for ReadValueId {
    type Inner = UA_ReadValueId;

    const INNER: u32 = UA_TYPES_READVALUEID;

    fn as_ptr(&self) -> *const Self::Inner {
        ReadValueId::as_ptr(self)
    }
}

impl ReadValueId {
    #[must_use]
    pub fn node_id(mut self, node_id: &ua::NodeId) -> Self {
        let node_id = node_id.clone();

        unsafe { UA_NodeId_clear(ptr::addr_of_mut!(self.0.nodeId)) }
        self.0.nodeId = node_id.into_inner();

        self
    }

    #[must_use]
    pub fn attribute_id(mut self, attribute_id: u32) -> Self {
        self.0.attributeId = attribute_id;
        self
    }
}
