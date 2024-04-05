use std::ptr::{self, NonNull};

use open62541_sys::{UA_Server, UA_StatusCode};

use crate::{DataType, Error};

use super::StatusCode;

use crate::ua;

/// Wrapper for [`UA_Server`] from [`open62541_sys`].
///
/// This owns the wrapped data type. When the wrapper is dropped, its inner value is cleaned up with
/// [`open62541_sys::UA_Server_delete()`].
pub struct Server(NonNull<UA_Server>);

impl Server {
    /// Creates a new server.
    ///
    /// # Panics
    ///
    /// When out of memory
    #[must_use]
    pub(crate) fn new() -> Self {
        let inner = unsafe { open62541_sys::UA_Server_new() };
        // PANIC: The only possible errors here are out-of-memory.
        let inner = NonNull::new(inner).expect("create UA_Server");
        Self(inner)
    }

    /// Returns mutable pointer to value.
    ///
    /// # Safety
    ///
    /// The value is owned by `Self`. Ownership must not be given away, in whole or in parts. This
    /// may happen when `open62541` functions are called that take ownership of values by pointer.
    #[must_use]
    pub(crate) const unsafe fn as_mut_ptr(&self) -> *mut UA_Server {
        self.0.as_ptr()
    }

    /// Wrapper function for `open62541_sys::UA_Server_runUntilInterrupt`
    #[must_use]
    pub(crate) fn run_until_interrupt(self) -> StatusCode {
        let ua_status_code: UA_StatusCode =
            unsafe { open62541_sys::UA_Server_runUntilInterrupt(self.as_mut_ptr()) };
        StatusCode::new(ua_status_code)
    }

    pub(crate) fn add_variable_node(
        &mut self,
        requested_new_node_id: ua::NodeId,
        parent_node_id: ua::NodeId,
        reference_type_id: ua::NodeId,
        browse_name: ua::QualifiedName,
        type_definition: ua::NodeId,
        attrs: ua::VariableAttributes,
    ) -> StatusCode {
        let status_code: UA_StatusCode = unsafe {
            open62541_sys::UA_Server_addVariableNode(
                self.as_mut_ptr(),
                requested_new_node_id.into_raw(),
                parent_node_id.into_raw(),
                reference_type_id.into_raw(),
                browse_name.into_raw(),
                type_definition.into_raw(),
                attrs.into_raw(),
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };
        StatusCode::new(status_code)
    }

    pub(crate) fn add_object_node(
        &mut self,
        requested_new_node_id: ua::NodeId,
        parent_node_id: ua::NodeId,
        reference_type_id: ua::NodeId,
        browse_name: ua::QualifiedName,
        type_definition: ua::NodeId,
        attrs: ua::ObjectAttributes,
    ) -> StatusCode {
        let status_code: UA_StatusCode = unsafe {
            open62541_sys::UA_Server_addObjectNode(
                self.as_mut_ptr(),
                requested_new_node_id.into_raw(),
                parent_node_id.into_raw(),
                reference_type_id.into_raw(),
                browse_name.into_raw(),
                type_definition.into_raw(),
                attrs.into_raw(),
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };
        StatusCode::new(status_code)
    }

    pub(crate) fn write_variable(&mut self, node_id: ua::NodeId, value: ua::Variant) -> StatusCode {
        let status_code = unsafe {
            open62541_sys::UA_Server_writeValue(
                self.as_mut_ptr(),
                node_id.into_raw(),
                value.into_raw(),
            )
        };
        StatusCode::new(status_code)
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        log::debug!("Deleting server");
        let status_code: UA_StatusCode =
            unsafe { open62541_sys::UA_Server_delete(self.as_mut_ptr()) };

        let status_code = StatusCode::new(status_code);

        if let Err(error) = Error::verify_good(&status_code) {
            log::warn!("Error while dropping server: {error}");
        }
    }
}
