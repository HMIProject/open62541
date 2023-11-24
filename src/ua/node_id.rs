use std::{ffi::CString, fmt, mem, ptr};

use open62541_sys::{
    UA_NodeId, UA_NodeIdType_UA_NODEIDTYPE_NUMERIC, UA_NodeIdType_UA_NODEIDTYPE_STRING,
    UA_NodeId_clear, UA_NodeId_copy, UA_NodeId_init, UA_NodeId_print, UA_NODEID_NUMERIC,
    UA_NODEID_STRING_ALLOC, UA_STATUSCODE_GOOD,
};

use crate::ua;

pub struct NodeId(UA_NodeId);

impl NodeId {
    #[must_use]
    pub fn new() -> Self {
        let mut inner = unsafe { mem::MaybeUninit::<UA_NodeId>::zeroed().assume_init() };
        unsafe { UA_NodeId_init(ptr::addr_of_mut!(inner)) }
        Self(inner)
    }

    /// Copies value from `src`.
    #[allow(dead_code)]
    pub(crate) fn from(src: &UA_NodeId) -> Self {
        let mut dst = Self::new();

        let result = unsafe { UA_NodeId_copy(src, dst.as_mut_ptr()) };
        assert_eq!(result, UA_STATUSCODE_GOOD);

        dst
    }

    /// Takes ownership of `src`.
    #[allow(dead_code)]
    pub(crate) fn from_inner(src: UA_NodeId) -> Self {
        Self(src)
    }

    #[allow(dead_code)]
    pub(crate) const fn as_ref(&self) -> &UA_NodeId {
        &self.0
    }

    #[allow(dead_code)]
    pub(crate) fn as_mut(&mut self) -> &mut UA_NodeId {
        &mut self.0
    }

    #[allow(dead_code)]
    pub(crate) const fn as_ptr(&self) -> *const UA_NodeId {
        ptr::addr_of!(self.0)
    }

    #[allow(dead_code)]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut UA_NodeId {
        ptr::addr_of_mut!(self.0)
    }

    #[allow(dead_code)]
    pub(crate) fn into_inner(self) -> UA_NodeId {
        let inner = self.0;
        mem::forget(self);
        inner
    }
}

impl Drop for NodeId {
    fn drop(&mut self) {
        unsafe { UA_NodeId_clear(self.as_mut_ptr()) }
    }
}

impl Default for NodeId {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for NodeId {
    fn clone(&self) -> Self {
        let mut dst = Self::new();

        let result = unsafe { UA_NodeId_copy(self.as_ptr(), dst.as_mut_ptr()) };
        assert_eq!(result, UA_STATUSCODE_GOOD);

        dst
    }
}

impl fmt::Debug for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = ua::String::new();

        let result = unsafe { UA_NodeId_print(self.as_ptr(), output.as_mut_ptr()) };
        if result != UA_STATUSCODE_GOOD {
            return f.write_str("NodeId");
        }

        match output.as_str() {
            Some(str) => f.write_str(str),
            None => f.write_str("NodeId"),
        }
    }
}

impl NodeId {
    #[must_use]
    pub fn new_numeric(ns_index: u16, numeric: u32) -> Option<Self> {
        let inner = unsafe { UA_NODEID_NUMERIC(ns_index, numeric) };

        debug_assert_eq!(inner.identifierType, UA_NodeIdType_UA_NODEIDTYPE_NUMERIC);
        if inner.identifierType != UA_NodeIdType_UA_NODEIDTYPE_NUMERIC {
            return None;
        }

        Some(Self(inner))
    }

    #[must_use]
    pub fn new_string(ns_index: u16, string: &str) -> Option<Self> {
        let string = CString::new(string).ok()?;

        // Technically, string allocation may fail but `UA_NODEID_STRING_ALLOC` doesn't tell us that
        // when it happens. Instead, we end up with a well-defined node ID that has an empty string.
        let inner = unsafe { UA_NODEID_STRING_ALLOC(ns_index, string.as_ptr()) };

        debug_assert_eq!(inner.identifierType, UA_NodeIdType_UA_NODEIDTYPE_STRING);
        if inner.identifierType != UA_NodeIdType_UA_NODEIDTYPE_STRING {
            return None;
        }

        let string = &unsafe { inner.identifier.string };
        if string.data.is_null() || string.length == 0 {
            debug_assert!(string.data.is_null());

            return None;
        }

        Some(Self(inner))
    }
}
