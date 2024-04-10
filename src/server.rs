use std::ptr;

use open62541_sys::{
    UA_Server_addObjectNode, UA_Server_addVariableNode, UA_Server_runUntilInterrupt,
};

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

    /// Adds variable to address space.
    ///
    /// # Errors
    ///
    /// This fails when the node cannot be added.
    pub fn add_variable_node(&mut self, node: VariableNode) -> Result<()> {
        let status_code = ua::StatusCode::new(unsafe {
            UA_Server_addVariableNode(
                self.0.as_mut_ptr(),
                node.requested_new_node_id.into_raw(),
                node.parent_node_id.into_raw(),
                node.reference_type_id.into_raw(),
                node.browse_name.into_raw(),
                node.type_definition.into_raw(),
                node.attributes.into_raw(),
                ptr::null_mut(),
                ptr::null_mut(),
            )
        });
        Error::verify_good(&status_code)
    }

    /// Adds object to address space.
    ///
    /// # Errors
    ///
    /// This fails when the node cannot be added.
    pub fn add_object_node(&mut self, node: ObjectNode) -> Result<()> {
        let status_code = ua::StatusCode::new(unsafe {
            UA_Server_addObjectNode(
                self.0.as_mut_ptr(),
                node.requested_new_node_id.into_raw(),
                node.parent_node_id.into_raw(),
                node.reference_type_id.into_raw(),
                node.browse_name.into_raw(),
                node.type_definition.into_raw(),
                node.attributes.into_raw(),
                ptr::null_mut(),
                ptr::null_mut(),
            )
        });
        Error::verify_good(&status_code)
    }

    /// Writes value to variable.
    ///
    /// # Errors
    ///
    /// This fails when the variable cannot be written.
    pub fn write_variable(&mut self, node_id: ua::NodeId, value: ua::Variant) -> Result<()> {
        let status_code = ua::StatusCode::new(unsafe {
            open62541_sys::UA_Server_writeValue(
                self.0.as_mut_ptr(),
                node_id.into_raw(),
                value.into_raw(),
            )
        });
        Error::verify_good(&status_code)
    }

    /// Writes string to variable.
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
    /// This fails when the variable cannot be written.
    pub fn write_variable_string(&mut self, node_id: ua::NodeId, value: &str) -> Result<()> {
        let value = ua::String::new(value)?;
        let ua_variant = ua::Variant::init().with_scalar(&value);
        self.write_variable(node_id, ua_variant)
    }

    /// Runs the server until interrupted.
    ///
    /// # Errors
    ///
    /// When an error occurred internally
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
