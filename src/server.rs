use std::{ffi::c_void, ptr};

use open62541_sys::{UA_Server_runUntilInterrupt, __UA_Server_addNode, __UA_Server_write};

use crate::{ua, DataType, Error, ObjectNode, Result, VariableNode};

/// OPC UA server.
///
/// This represents an OPC UA server. Nodes can be added through the several methods below, and then
/// the server can be started with [`run()`](Self::run).
pub struct Server(ua::Server);

impl Server {
    /// Creates server.
    #[must_use]
    pub fn new() -> Self {
        Self(ua::Server::new())
    }

    /// Adds object node to address space.
    ///
    /// # Errors
    ///
    /// This fails when the node cannot be added.
    pub fn add_object_node(&mut self, node: ObjectNode) -> Result<()> {
        let status_code = ua::StatusCode::new(unsafe {
            __UA_Server_addNode(
                self.0.as_mut_ptr(),
                // Passing ownership is trivial with primitive value (`u32`).
                ua::NodeClass::OBJECT.into_raw(),
                node.requested_new_node_id.as_ptr(),
                node.parent_node_id.as_ptr(),
                node.reference_type_id.as_ptr(),
                // TODO: Verify that `__UA_Server_addNode()` takes ownership.
                node.browse_name.into_raw(),
                node.type_definition.as_ptr(),
                node.attributes.as_node_attributes().as_ptr(),
                ua::ObjectAttributes::data_type(),
                ptr::null_mut(),
                ptr::null_mut(),
            )
        });
        Error::verify_good(&status_code)
    }

    /// Adds variable node to address space.
    ///
    /// # Errors
    ///
    /// This fails when the node cannot be added.
    pub fn add_variable_node(&mut self, node: VariableNode) -> Result<()> {
        let status_code = ua::StatusCode::new(unsafe {
            __UA_Server_addNode(
                self.0.as_mut_ptr(),
                // Passing ownership is trivial with primitive value (`u32`).
                ua::NodeClass::VARIABLE.into_raw(),
                node.requested_new_node_id.as_ptr(),
                node.parent_node_id.as_ptr(),
                node.reference_type_id.as_ptr(),
                // TODO: Verify that `__UA_Server_addNode()` takes ownership.
                node.browse_name.into_raw(),
                node.type_definition.as_ptr(),
                node.attributes.as_node_attributes().as_ptr(),
                ua::VariableAttributes::data_type(),
                ptr::null_mut(),
                ptr::null_mut(),
            )
        });
        Error::verify_good(&status_code)
    }

    /// Writes value to variable node.
    ///
    /// # Errors
    ///
    /// This fails when the variable node cannot be written.
    pub fn write_variable(&mut self, node_id: ua::NodeId, value: ua::Variant) -> Result<()> {
        let status_code = ua::StatusCode::new(unsafe {
            __UA_Server_write(
                self.0.as_mut_ptr(),
                node_id.as_ptr(),
                // Passing ownership is trivial with primitive value (`u32`).
                ua::AttributeId::VALUE.into_raw(),
                ua::Variant::data_type(),
                value.as_ptr().cast::<c_void>(),
            )
        });
        Error::verify_good(&status_code)
    }

    /// Writes string value to variable node.
    ///
    /// This is a shortcut and roughly equivalent to the following:
    ///
    /// ```
    /// # use open62541::{ua, DataType as _, Server};
    /// #
    /// # fn write_string(
    /// #     server: &mut Server,
    /// #     node_id: ua::NodeId,
    /// #     value: &str,
    /// # ) -> anyhow::Result<()> {
    /// let value = ua::String::new(value)?;
    /// let value = ua::Variant::init().with_scalar(&value);
    /// server.write_variable(node_id, value)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This fails when the variable node cannot be written.
    pub fn write_variable_string(&mut self, node_id: ua::NodeId, value: &str) -> Result<()> {
        let value = ua::String::new(value)?;
        let ua_variant = ua::Variant::init().with_scalar(&value);
        self.write_variable(node_id, ua_variant)
    }

    /// Runs the server until interrupted.
    ///
    /// The server is shut down cleanly upon receiving the `SIGINT` signal at which point the method
    /// returns.
    ///
    /// # Errors
    ///
    /// This fails when the server cannot be started.
    pub fn run(self) -> Result<()> {
        let status_code =
            ua::StatusCode::new(unsafe { UA_Server_runUntilInterrupt(self.0.as_mut_ptr()) });
        Error::verify_good(&status_code)
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}
