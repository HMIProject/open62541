use open62541_sys::{
    UA_Boolean, UA_DataSource, UA_DataValue, UA_NodeId, UA_NumericRange, UA_Server, UA_StatusCode,
};

// Safe wrapper struct around UA_DataSource
#[derive(Debug)]
pub struct DataSource(pub UA_DataSource);

impl DataSource {
    #[must_use]
    pub fn new(
        read: Option<
            unsafe extern "C" fn(
                *mut UA_Server,
                *const UA_NodeId,
                *mut ::core::ffi::c_void,
                *const UA_NodeId,
                *mut ::core::ffi::c_void,
                UA_Boolean,
                *const UA_NumericRange,
                *mut UA_DataValue,
            ) -> UA_StatusCode,
        >,
        write: Option<
            unsafe extern "C" fn(
                *mut UA_Server,
                *const UA_NodeId,
                *mut ::core::ffi::c_void,
                *const UA_NodeId,
                *mut ::core::ffi::c_void,
                *const UA_NumericRange,
                *const UA_DataValue,
            ) -> UA_StatusCode,
        >,
    ) -> Self {
        DataSource(UA_DataSource { read, write })
    }

    #[must_use]
    pub const fn inner(&self) -> &UA_DataSource {
        &self.0
    }

    #[must_use]
    pub const fn into_raw(self) -> UA_DataSource {
        // SAFETY: Move value out of `self` despite it not being `Copy`. We consume `self`
        // and forget it below, so that `Drop` is not called on the original value.
        unsafe { std::ptr::read(std::ptr::addr_of!(self.0)) }
    }
}

impl Clone for DataSource {
    fn clone(&self) -> Self {
        println!("clone is working");
        DataSource(UA_DataSource {
            read: self.0.read,
            write: self.0.write,
        })
    }
}
