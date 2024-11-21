mod access_control;
mod data_source;
mod method_callback;
mod node_context;
mod node_types;

use std::{
    any::Any,
    ffi::{c_void, CString},
    ptr,
    sync::Arc,
    time::Instant,
};

use open62541_sys::{
    UA_CertificateVerification_AcceptAll, UA_NodeId, UA_Server, UA_ServerConfig,
    UA_Server_addDataSourceVariableNode, UA_Server_addMethodNodeEx, UA_Server_addNamespace,
    UA_Server_addReference, UA_Server_browse, UA_Server_browseNext, UA_Server_browseRecursive,
    UA_Server_browseSimplifiedBrowsePath, UA_Server_createEvent, UA_Server_deleteNode,
    UA_Server_deleteReference, UA_Server_getNamespaceByIndex, UA_Server_getNamespaceByName,
    UA_Server_read, UA_Server_readObjectProperty, UA_Server_runUntilInterrupt,
    UA_Server_translateBrowsePathToNodeIds, UA_Server_triggerEvent, UA_Server_writeObjectProperty,
    __UA_Server_addNode, __UA_Server_write, UA_STATUSCODE_BADNOTFOUND,
};

use crate::{
    ua, Attribute, Attributes, BrowseResult, DataType, DataValue, Error, Result,
    DEFAULT_PORT_NUMBER,
};

pub(crate) use self::node_context::NodeContext;
pub use self::{
    access_control::{AccessControl, DefaultAccessControl, DefaultAccessControlWithLoginCallback},
    data_source::{
        DataSource, DataSourceError, DataSourceReadContext, DataSourceResult,
        DataSourceWriteContext,
    },
    method_callback::{
        MethodCallback, MethodCallbackContext, MethodCallbackError, MethodCallbackResult,
    },
    node_types::{MethodNode, Node, ObjectNode, VariableNode},
};

/// Builder for [`Server`].
///
/// Use this to specify additional options when building an OPC UA server.
///
/// # Examples
///
/// ```
/// use open62541::ServerBuilder;
///
/// # #[tokio::main]
/// # async fn main() -> anyhow::Result<()> {
/// #
/// let (server, runner) = ServerBuilder::default()
///     .server_urls(&["opc.tcp://localhost:4840"])
///     .build();
/// #
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct ServerBuilder {
    config: ua::ServerConfig,

    /// [`AccessControl`] instances may hold additional data that must be kept alive until server is
    /// shut down. The sentinel value cleans this up when it is dropped.
    access_control_sentinel: Option<Box<dyn Any + Send>>,
}

impl ServerBuilder {
    fn new(config: ua::ServerConfig) -> Self {
        Self {
            config,
            access_control_sentinel: None,
        }
    }

    /// Creates builder from minimal server config.
    // Method name refers to call of `UA_ServerConfig_setMinimal()`.
    #[must_use]
    pub fn minimal(port_number: u16, certificate: Option<&[u8]>) -> Self {
        Self::new(ua::ServerConfig::minimal(port_number, certificate))
    }

    /// Creates builder from default server config with security policies.
    ///
    /// This enables both secure and non-secure (i.e. unencrypted) security policies. If only secure
    /// security policies should be activated, use [`Self::default_with_secure_security_policies()`]
    /// instead.
    ///
    /// This requires certificate and associated private key data in binary format. For convenience,
    /// consider reading those from PEM text files using the [pem] crate or other suitable crates:
    ///
    /// [pem]: https://crates.io/crates/pem
    ///
    /// ```
    /// use open62541::{DEFAULT_PORT_NUMBER, ServerBuilder};
    ///
    /// const CERTIFICATE_PEM: &'static str = include_str!("../examples/server_certificate.pem");
    /// const PRIVATE_KEY_PEM: &'static str = include_str!("../examples/server_private_key.pem");
    ///
    /// let certificate = pem::parse(CERTIFICATE_PEM).expect("should parse PEM certificate");
    /// let private_key = pem::parse(PRIVATE_KEY_PEM).expect("should parse PEM private key");
    ///
    /// let server = ServerBuilder::default_with_security_policies(
    ///     DEFAULT_PORT_NUMBER,
    ///     certificate.contents(),
    ///     private_key.contents(),
    /// )
    /// .expect("should create builder with security policies")
    /// .build();
    /// ```
    ///
    /// # Errors
    ///
    /// This fails when the certificate is invalid or the private key cannot be decrypted (e.g. when
    /// it has been protected by a password).
    // Method name refers to call of `UA_ServerConfig_setDefaultWithSecurityPolicies()`.
    #[cfg(feature = "mbedtls")]
    pub fn default_with_security_policies(
        port_number: u16,
        certificate: &[u8],
        private_key: &[u8],
    ) -> Result<Self> {
        Ok(Self::new(ua::ServerConfig::default_with_security_policies(
            port_number,
            certificate,
            private_key,
        )?))
    }

    /// Creates builder from default server config with secure security policies.
    ///
    /// This enables only secure (i.e. encrypted) security policies.
    ///
    /// See also [`Self::default_with_security_policies()`].
    ///
    /// # Errors
    ///
    /// This fails when the certificate is invalid or the private key cannot be decrypted (e.g. when
    /// it has been protected by a password).
    // Method name refers to call of `UA_ServerConfig_setDefaultWithSecureSecurityPolicies()`.
    #[cfg(feature = "mbedtls")]
    pub fn default_with_secure_security_policies(
        port_number: u16,
        certificate: &[u8],
        private_key: &[u8],
    ) -> Result<Self> {
        Ok(Self::new(
            ua::ServerConfig::default_with_secure_security_policies(
                port_number,
                certificate,
                private_key,
            )?,
        ))
    }

    /// Sets server port number.
    ///
    /// This is a shortcut for setting the corresponding server URL `opc.tcp://:<port>` and thus may
    /// overwrite any previously set server URLs from [`server_urls()`](Self::server_urls).
    #[must_use]
    pub fn port(self, port_number: u16) -> Self {
        self.server_urls(&[&format!("opc.tcp://:{port_number}")])
    }

    /// Sets server URLs.
    ///
    /// # Panics
    ///
    /// The strings must not contain any NUL bytes.
    #[must_use]
    pub fn server_urls(mut self, server_urls: &[&str]) -> Self {
        let config = self.config_mut();
        let server_urls = server_urls
            .iter()
            .map(|server_url| ua::String::new(server_url).unwrap());
        ua::Array::from_iter(server_urls)
            .move_into_raw(&mut config.serverUrlsSize, &mut config.serverUrls);
        self
    }

    /// Disables client certificate checks.
    ///
    /// Note that this disables all certificate verification of client communications. Use only when
    /// clients can be identified in some other way, or identity is not relevant.
    #[must_use]
    pub fn accept_all(mut self) -> Self {
        let config = self.config_mut();
        unsafe {
            UA_CertificateVerification_AcceptAll(&mut config.secureChannelPKI);
            UA_CertificateVerification_AcceptAll(&mut config.sessionPKI);
        }
        self
    }

    /// Applies access control.
    ///
    /// See [`AccessControl`] for available implementations.
    ///
    /// # Errors
    ///
    /// This fails when the access control cannot be applied.
    pub fn access_control(mut self, access_control: impl AccessControl) -> Result<Self> {
        let config = self.config_mut();

        // SAFETY: We keep track of the returned sentinel value and drop it only when the server (to
        // be created from this config) is shut down and does not access this data anymore. If we do
        // not create a server from this config, the data can be released when dropping the builder.
        let sentinel = unsafe { access_control.apply(config) }?;

        // This may replace previously tracked sentinels. This is okay because `apply()` always must
        // replace the entire access control config. Thus, dropping the sentinel and cleaning up any
        // _previously_ set access control instance is okay.
        self.access_control_sentinel = Some(Box::new(sentinel));

        Ok(self)
    }

    /// Builds OPC UA server.
    #[must_use]
    pub fn build(mut self) -> (Server, ServerRunner) {
        unsafe extern "C" fn destructor_c(
            _server: *mut UA_Server,
            _session_id: *const UA_NodeId,
            _session_context: *mut c_void,
            node_id: *const UA_NodeId,
            node_context: *mut c_void,
        ) {
            // When associating dynamically allocated data with nodes created by this server, we
            // always use `NodeContext`. Therefore, if `node_context` is set at all, we can/must
            // call `NodeContext::consume()` to release that data. No other data must have been
            // stored inside `node_context`.
            //
            // Note: The above assumption is not correct. See issue for more details:
            // <https://github.com/HMIProject/open62541/issues/125>
            //
            // FIXME: Find solution to prevent memory leak.
            if !node_context.is_null() {
                if let Some(node_id) = unsafe { node_id.as_ref() }.map(ua::NodeId::raw_ref) {
                    log::debug!("Destroying node {node_id}, freeing associated data");
                } else {
                    log::debug!("Destroying node, freeing associated data");
                }
                // SAFETY: The node destructor is run only once and we never consume the context
                // outside of it.
                //
                // Note: We must not consume the node context because we cannot be sure that it
                // points to valid memory (see above). We leak memory here. Fix this soon.
                //
                // unsafe {
                //     let _unused = NodeContext::consume(node_context);
                // }
            }
        }

        let config = self.config_mut();

        // PANIC: We never set lifecycle hooks elsewhere in config.
        debug_assert!(config.nodeLifecycle.destructor.is_none());
        config.nodeLifecycle.destructor = Some(destructor_c);

        let Self {
            config,
            access_control_sentinel,
        } = self;

        let server = Arc::new(ua::Server::new_with_config(config));

        let runner = ServerRunner::new(&server, access_control_sentinel);
        let server = Server(server);
        (server, runner)
    }

    /// Access server configuration.
    fn config_mut(&mut self) -> &mut UA_ServerConfig {
        // SAFETY: Ownership is not given away.
        unsafe { self.config.as_mut() }
    }
}

impl Default for ServerBuilder {
    fn default() -> Self {
        Self::minimal(DEFAULT_PORT_NUMBER, None)
    }
}

/// OPC UA server.
///
/// This represents an OPC UA server. Nodes can be added through the several methods below.
///
/// Note: The server must be started with [`ServerRunner::run()`] before it can accept connections
/// from clients.
#[derive(Debug, Clone)]
pub struct Server(Arc<ua::Server>);

impl Server {
    /// Creates default server.
    ///
    /// If you need more control over the initialization, use [`ServerBuilder`] instead, and turn it
    /// into [`Server`](crate::Server) by calling [`build()`](ServerBuilder::build).
    ///
    /// # Errors
    ///
    /// See [`ServerBuilder::build()`].
    ///
    /// # Panics
    ///
    /// See [`ServerBuilder::build()`].
    #[must_use]
    pub fn new() -> (Self, ServerRunner) {
        ServerBuilder::default().build()
    }

    /// Adds a new namespace to the server. Returns the index of the new namespace.
    ///
    /// If the namespace already exists, it is not re-created but its index is returned.
    ///
    /// # Panics
    ///
    /// The namespace URI must not contain any NUL bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use open62541::ServerBuilder;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let (server, _) = ServerBuilder::default().build();
    /// #
    /// let ns_index = server.add_namespace("http://hmi-project.com/UA/");
    ///
    /// // Application URI takes index 1, new namespaces start at index 2.
    /// assert!(ns_index >= 2);
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn add_namespace(&self, namespace_uri: &str) -> u16 {
        let name = CString::new(namespace_uri).expect("namespace URI does not contain NUL bytes");
        let result = unsafe {
            UA_Server_addNamespace(
                // SAFETY: Cast to `mut` pointer, function is marked `UA_THREADSAFE`.
                self.0.as_ptr().cast_mut(),
                name.as_ptr(),
            )
        };
        // PANIC: The only possible errors here are out-of-memory.
        assert!(result != 0, "namespace should have been added");
        result
    }

    /// Looks up namespace by its URI.
    ///
    /// This returns the found namespace index.
    ///
    /// # Examples
    ///
    /// ```
    /// # use open62541::{ServerBuilder, ua};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let (server, _) = ServerBuilder::default().build();
    /// #
    /// let ns_index = server.add_namespace("http://hmi-project.com/UA/");
    ///
    /// let ns_uri = ua::String::new("http://hmi-project.com/UA/").unwrap();
    /// assert_eq!(server.get_namespace_by_name(&ns_uri), Some(ns_index));
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn get_namespace_by_name(&self, namespace_uri: &ua::String) -> Option<u16> {
        let mut found_index = 0;
        let status_code = ua::StatusCode::new(unsafe {
            UA_Server_getNamespaceByName(
                // SAFETY: Cast to `mut` pointer, function is marked `UA_THREADSAFE`.
                self.0.as_ptr().cast_mut(),
                // SAFETY: The `String` is used for comparison with internal strings only. It is not
                // changed and it is only used in the scope of the function. This means ownership is
                // preserved and passing by value is safe here.
                DataType::to_raw_copy(namespace_uri),
                ptr::addr_of_mut!(found_index),
            )
        });
        if !status_code.is_good() {
            debug_assert_eq!(status_code.code(), UA_STATUSCODE_BADNOTFOUND);
            return None;
        }
        // Namespace index is always expected to fit into `u16`.
        found_index.try_into().ok()
    }

    /// Looks up namespace by its index.
    ///
    /// This returns the found namespace URI.
    ///
    /// # Examples
    ///
    /// ```
    /// # use open62541::{ServerBuilder, ua};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let (server, _) = ServerBuilder::default().build();
    /// #
    /// let ns_index = server.add_namespace("http://hmi-project.com/UA/");
    ///
    /// let ns_uri = ua::String::new("http://hmi-project.com/UA/").unwrap();
    /// assert_eq!(server.get_namespace_by_index(ns_index), Some(ns_uri));
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Namespace index 0 is always the OPC UA namespace with a fixed URI:
    ///
    /// ```
    /// # use open62541::{ServerBuilder, ua};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let (server, _) = ServerBuilder::default().build();
    /// #
    /// let ns_uri = ua::String::new("http://opcfoundation.org/UA/").unwrap();
    /// assert_eq!(server.get_namespace_by_index(0), Some(ns_uri));
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn get_namespace_by_index(&self, namespace_index: u16) -> Option<ua::String> {
        let mut found_uri = ua::String::init();
        let status_code = ua::StatusCode::new(unsafe {
            UA_Server_getNamespaceByIndex(
                // SAFETY: Cast to `mut` pointer, function is marked `UA_THREADSAFE`.
                self.0.as_ptr().cast_mut(),
                namespace_index.into(),
                found_uri.as_mut_ptr(),
            )
        });
        if !status_code.is_good() {
            // PANIC: The only other possible errors here are out-of-memory.
            debug_assert_eq!(status_code.code(), UA_STATUSCODE_BADNOTFOUND);
            return None;
        }
        Some(found_uri)
    }

    /// Adds node to address space.
    ///
    /// This returns the node ID that was actually inserted (when no explicit requested new node ID
    /// was given in `node`).
    ///
    /// # Errors
    ///
    /// This fails when the node cannot be added.
    pub fn add_node<T: Attributes>(&self, node: Node<T>) -> Result<ua::NodeId> {
        let Node {
            requested_new_node_id,
            parent_node_id,
            reference_type_id,
            browse_name,
            type_definition,
            attributes,
            context,
        } = node;

        let requested_new_node_id = requested_new_node_id.unwrap_or(ua::NodeId::null());

        // This out variable must be initialized without memory allocation because the call below
        // overwrites it in place, without releasing any held data first.
        let mut out_new_node_id = ua::NodeId::null();

        let status_code = ua::StatusCode::new(unsafe {
            __UA_Server_addNode(
                // SAFETY: Cast to `mut` pointer, function is marked `UA_THREADSAFE`.
                self.0.as_ptr().cast_mut(),
                // Passing ownership is trivial with primitive value (`u32`).
                attributes.node_class().clone().into_raw(),
                requested_new_node_id.as_ptr(),
                parent_node_id.as_ptr(),
                reference_type_id.as_ptr(),
                // TODO: Verify that `__UA_Server_addNode()` takes ownership.
                browse_name.clone().into_raw(),
                type_definition.as_ptr(),
                attributes.as_node_attributes().as_ptr(),
                attributes.attribute_type(),
                context.map_or(ptr::null_mut(), NodeContext::leak),
                out_new_node_id.as_mut_ptr(),
            )
        });
        Error::verify_good(&status_code)?;

        Ok(out_new_node_id)
    }

    /// Adds object node to address space.
    ///
    /// This returns the node ID that was actually inserted (when no explicit requested new node ID
    /// was given in `node`).
    ///
    /// # Errors
    ///
    /// This fails when the node cannot be added.
    pub fn add_object_node(&self, object_node: ObjectNode) -> Result<ua::NodeId> {
        let ObjectNode {
            requested_new_node_id,
            parent_node_id,
            reference_type_id,
            browse_name,
            type_definition,
            attributes,
        } = object_node;

        let requested_new_node_id = requested_new_node_id.unwrap_or(ua::NodeId::null());

        // This out variable must be initialized without memory allocation because the call below
        // overwrites it in place, without releasing any held data first.
        let mut out_new_node_id = ua::NodeId::null();

        let status_code = ua::StatusCode::new(unsafe {
            __UA_Server_addNode(
                // SAFETY: Cast to `mut` pointer, function is marked `UA_THREADSAFE`.
                self.0.as_ptr().cast_mut(),
                // Passing ownership is trivial with primitive value (`u32`).
                ua::NodeClass::OBJECT.into_raw(),
                requested_new_node_id.as_ptr(),
                parent_node_id.as_ptr(),
                reference_type_id.as_ptr(),
                // TODO: Verify that `__UA_Server_addNode()` takes ownership.
                browse_name.into_raw(),
                type_definition.as_ptr(),
                attributes.as_node_attributes().as_ptr(),
                ua::ObjectAttributes::data_type(),
                ptr::null_mut(),
                out_new_node_id.as_mut_ptr(),
            )
        });
        Error::verify_good(&status_code)?;

        Ok(out_new_node_id)
    }

    /// Adds variable node to address space.
    ///
    /// This returns the node ID that was actually inserted (when no explicit requested new node ID
    /// was given in `node`).
    ///
    /// # Errors
    ///
    /// This fails when the node cannot be added.
    pub fn add_variable_node(&self, variable_node: VariableNode) -> Result<ua::NodeId> {
        let VariableNode {
            requested_new_node_id,
            parent_node_id,
            reference_type_id,
            browse_name,
            type_definition,
            attributes,
        } = variable_node;

        let requested_new_node_id = requested_new_node_id.unwrap_or(ua::NodeId::null());

        // This out variable must be initialized without memory allocation because the call below
        // overwrites it in place, without releasing any held data first.
        let mut out_new_node_id = ua::NodeId::null();

        let status_code = ua::StatusCode::new(unsafe {
            __UA_Server_addNode(
                // SAFETY: Cast to `mut` pointer, function is marked `UA_THREADSAFE`.
                self.0.as_ptr().cast_mut(),
                // Passing ownership is trivial with primitive value (`u32`).
                ua::NodeClass::VARIABLE.into_raw(),
                requested_new_node_id.as_ptr(),
                parent_node_id.as_ptr(),
                reference_type_id.as_ptr(),
                // TODO: Verify that `__UA_Server_addNode()` takes ownership.
                browse_name.into_raw(),
                type_definition.as_ptr(),
                attributes.as_node_attributes().as_ptr(),
                ua::VariableAttributes::data_type(),
                ptr::null_mut(),
                out_new_node_id.as_mut_ptr(),
            )
        });
        Error::verify_good(&status_code)?;

        Ok(out_new_node_id)
    }

    /// Adds variable node with data source to address space.
    ///
    /// This returns the node ID that was actually inserted (when no explicit requested new node ID
    /// was given in `node`).
    ///
    /// # Errors
    ///
    /// This fails when the node cannot be added.
    pub fn add_data_source_variable_node(
        &self,
        variable_node: VariableNode,
        data_source: impl DataSource + 'static,
    ) -> Result<ua::NodeId> {
        let VariableNode {
            requested_new_node_id,
            parent_node_id,
            reference_type_id,
            browse_name,
            type_definition,
            attributes,
        } = variable_node;

        let requested_new_node_id = requested_new_node_id.unwrap_or(ua::NodeId::null());

        // This out variable must be initialized without memory allocation because the call below
        // overwrites it in place, without releasing any held data first.
        let mut out_new_node_id = ua::NodeId::null();

        // SAFETY: We store `node_context` inside the node to keep `data_source` alive.
        let (data_source, node_context) = unsafe { data_source::wrap_data_source(data_source) };
        let status_code = ua::StatusCode::new(unsafe {
            UA_Server_addDataSourceVariableNode(
                // SAFETY: Cast to `mut` pointer, function is marked `UA_THREADSAFE`.
                self.0.as_ptr().cast_mut(),
                // TODO: Verify that `UA_Server_addDataSourceVariableNode()` takes ownership.
                requested_new_node_id.into_raw(),
                // TODO: Verify that `UA_Server_addDataSourceVariableNode()` takes ownership.
                parent_node_id.into_raw(),
                // TODO: Verify that `UA_Server_addDataSourceVariableNode()` takes ownership.
                reference_type_id.into_raw(),
                // TODO: Verify that `UA_Server_addDataSourceVariableNode()` takes ownership.
                browse_name.into_raw(),
                // TODO: Verify that `UA_Server_addDataSourceVariableNode()` takes ownership.
                type_definition.into_raw(),
                // TODO: Verify that `UA_Server_addDataSourceVariableNode()` takes ownership.
                attributes.into_raw(),
                data_source,
                node_context.leak(),
                out_new_node_id.as_mut_ptr(),
            )
        });
        // In case of an error, the node context has already been freed by the destructor. We must
        // not consume it ourselves (to avoid double-freeing). In case of success, the node context
        // will be consumed when the node is eventually deleted (`UA_ServerConfig::nodeLifecycle`).
        Error::verify_good(&status_code)?;

        Ok(out_new_node_id)
    }

    /// Adds method node to address space.
    ///
    /// This returns the node ID that was actually inserted (when no explicit requested new node ID
    /// was given in `node`), along with the node IDs for the input and output argument nodes.
    ///
    /// # Errors
    ///
    /// This fails when the node cannot be added.
    pub fn add_method_node(
        &self,
        method_node: MethodNode,
        callback: impl MethodCallback + 'static,
    ) -> Result<(ua::NodeId, (ua::NodeId, ua::NodeId))> {
        let MethodNode {
            requested_new_node_id,
            parent_node_id,
            reference_type_id,
            browse_name,
            attributes,
            input_arguments,
            input_arguments_requested_new_node_id,
            output_arguments,
            output_arguments_requested_new_node_id,
        } = method_node;

        let requested_new_node_id = requested_new_node_id.unwrap_or(ua::NodeId::null());

        // SAFETY: We store `node_context` inside the node to keep `data_source` alive.
        let (method_callback, node_context) =
            unsafe { method_callback::wrap_method_callback(callback) };

        let (input_arguments_size, input_arguments) = unsafe { input_arguments.as_raw_parts() };
        let (output_arguments_size, output_arguments) = unsafe { output_arguments.as_raw_parts() };

        // These out variables must be initialized without memory allocation because the call below
        // overwrites them in place, without releasing any held data first.
        let mut input_arguments_out_new_node_id = ua::NodeId::null();
        let mut output_arguments_out_new_node_id = ua::NodeId::null();
        let mut out_new_node_id = ua::NodeId::null();

        let status_code = ua::StatusCode::new(unsafe {
            UA_Server_addMethodNodeEx(
                // SAFETY: Cast to `mut` pointer, function is marked `UA_THREADSAFE`.
                self.0.as_ptr().cast_mut(),
                // TODO: Verify that `UA_Server_addMethodNodeEx()` takes ownership.
                requested_new_node_id.into_raw(),
                // TODO: Verify that `UA_Server_addMethodNodeEx()` takes ownership.
                parent_node_id.into_raw(),
                // TODO: Verify that `UA_Server_addMethodNodeEx()` takes ownership.
                reference_type_id.into_raw(),
                // TODO: Verify that `UA_Server_addMethodNodeEx()` takes ownership.
                browse_name.clone().into_raw(),
                // TODO: Verify that `UA_Server_addMethodNodeEx()` takes ownership.
                attributes.into_raw(),
                method_callback,
                input_arguments_size,
                input_arguments,
                input_arguments_requested_new_node_id
                    .unwrap_or_else(ua::NodeId::null)
                    // TODO: Verify that `UA_Server_addMethodNodeEx()` takes ownership.
                    .into_raw(),
                input_arguments_out_new_node_id.as_mut_ptr(),
                output_arguments_size,
                output_arguments,
                output_arguments_requested_new_node_id
                    .unwrap_or_else(ua::NodeId::null)
                    // TODO: Verify that `UA_Server_addMethodNodeEx()` takes ownership.
                    .into_raw(),
                output_arguments_out_new_node_id.as_mut_ptr(),
                node_context.leak(),
                out_new_node_id.as_mut_ptr(),
            )
        });
        Error::verify_good(&status_code)?;

        Ok((
            out_new_node_id,
            (
                input_arguments_out_new_node_id,
                output_arguments_out_new_node_id,
            ),
        ))
    }

    /// Deletes node from address space.
    ///
    /// This also deletes all references leading to the node.
    ///
    /// # Errors
    ///
    /// This fails when the node cannot be deleted.
    pub fn delete_node(&self, node_id: &ua::NodeId) -> Result<()> {
        let status_code = ua::StatusCode::new(unsafe {
            UA_Server_deleteNode(
                // SAFETY: Cast to `mut` pointer, function is marked `UA_THREADSAFE`.
                self.0.as_ptr().cast_mut(),
                // SAFETY: `UA_Server_deleteNode()` expects the node ID passed by value but does not
                // take ownership.
                ua::NodeId::to_raw_copy(node_id),
                // Delete all references to this node.
                true,
            )
        });
        Error::verify_good(&status_code)
    }

    /// Adds a reference from one node to another.
    ///
    /// # Errors
    ///
    /// This fails when adding the reference fails.
    ///
    /// # Examples
    ///
    /// ```
    /// # use open62541::{DataType as _, Node, ServerBuilder, ua};
    /// # use open62541_sys::{UA_NS0ID_HASCOMPONENT, UA_NS0ID_OBJECTSFOLDER};
    /// use open62541_sys::{UA_NS0ID_ORGANIZES};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let (server, _) = ServerBuilder::default().build();
    /// #
    /// // let parent_one_node_id = server.add_node(/* snip */)?;
    /// # let parent_one_node_id = server.add_node(Node::new(
    /// #     ua::NodeId::ns0(UA_NS0ID_OBJECTSFOLDER),
    /// #     ua::NodeId::ns0(UA_NS0ID_ORGANIZES),
    /// #     ua::QualifiedName::new(1, "ParentOne"),
    /// #     ua::ObjectAttributes::init(),
    /// # ))?;
    /// // let parent_two_node_id = server.add_node(/* snip */)?;
    /// # let parent_two_node_id = server.add_node(Node::new(
    /// #     ua::NodeId::ns0(UA_NS0ID_OBJECTSFOLDER),
    /// #     ua::NodeId::ns0(UA_NS0ID_ORGANIZES),
    /// #     ua::QualifiedName::new(1, "ParentTwo"),
    /// #     ua::ObjectAttributes::init(),
    /// # ))?;
    ///
    /// let variable_node_id = server.add_node(Node::new(
    ///     parent_one_node_id.clone(),
    ///     ua::NodeId::ns0(UA_NS0ID_ORGANIZES),
    ///     ua::QualifiedName::new(1, "Variable"),
    ///     ua::VariableAttributes::init(),
    /// ))?;
    ///
    /// // This makes the variable available in two parents.
    /// server.add_reference(
    ///     &parent_two_node_id,
    ///     &ua::NodeId::ns0(UA_NS0ID_ORGANIZES),
    ///     &variable_node_id.clone().into_expanded_node_id(),
    ///     true,
    /// )?;
    ///
    /// // Duplicating an existing reference is not allowed.
    /// let error = server.add_reference(
    ///     &parent_one_node_id,
    ///     &ua::NodeId::ns0(UA_NS0ID_ORGANIZES),
    ///     &variable_node_id.clone().into_expanded_node_id(),
    ///     true,
    /// ).unwrap_err();
    /// assert_eq!(error.status_code(), ua::StatusCode::BADDUPLICATEREFERENCENOTALLOWED);
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn add_reference(
        &self,
        source_id: &ua::NodeId,
        reference_type_id: &ua::NodeId,
        target_id: &ua::ExpandedNodeId,
        is_forward: bool,
    ) -> Result<()> {
        let status_code = ua::StatusCode::new(unsafe {
            UA_Server_addReference(
                // SAFETY: Cast to `mut` pointer, function is marked `UA_THREADSAFE`.
                self.0.as_ptr().cast_mut(),
                // SAFETY: The `NodeId` values are used to find internal pointers, are not modified
                // and no references to these variables exist beyond this function call. Passing by
                // value is safe here.
                DataType::to_raw_copy(source_id),
                DataType::to_raw_copy(reference_type_id),
                DataType::to_raw_copy(target_id),
                is_forward,
            )
        });
        Error::verify_good(&status_code)
    }

    /// Deletes a reference between two nodes.
    ///
    /// # Errors
    ///
    /// This fails when deleting the reference fails.
    pub fn delete_reference(
        &self,
        source_node_id: &ua::NodeId,
        reference_type_id: &ua::NodeId,
        target_node_id: &ua::ExpandedNodeId,
        is_forward: bool,
        delete_bidirectional: bool,
    ) -> Result<()> {
        let status_code = ua::StatusCode::new(unsafe {
            UA_Server_deleteReference(
                // SAFETY: Cast to `mut` pointer, function is marked `UA_THREADSAFE`.
                self.0.as_ptr().cast_mut(),
                // SAFETY: The `NodeId` values are used to find internal pointers, are not modified
                // and no references to these variables exist beyond this function call. Passing by
                // value is safe here.
                DataType::to_raw_copy(source_node_id),
                DataType::to_raw_copy(reference_type_id),
                is_forward,
                DataType::to_raw_copy(target_node_id),
                delete_bidirectional,
            )
        });
        Error::verify_good(&status_code)
    }

    /// Creates an event.
    ///
    /// This returns the [`ua::NodeId`] of the created event.
    ///
    /// # Errors
    ///
    /// This fails when the event could not be created.
    pub fn create_event(&self, event_type: &ua::NodeId) -> Result<ua::NodeId> {
        // This out variable must be initialized without memory allocation because the call below
        // overwrites it in place, without releasing any held data first.
        let mut out_node_id = ua::NodeId::init();
        let status_code = ua::StatusCode::new(unsafe {
            UA_Server_createEvent(
                // SAFETY: Cast to `mut` pointer, function is marked `UA_THREADSAFE`.
                self.0.as_ptr().cast_mut(),
                // SAFETY: Passing as value is okay here, as event_type is only used for the scope
                // of the function and does not get modified.
                DataType::to_raw_copy(event_type),
                out_node_id.as_mut_ptr(),
            )
        });
        Error::verify_good(&status_code)?;
        Ok(out_node_id)
    }

    /// Triggers an event.
    ///
    /// This returns the [`ua::EventId`] of the new event.
    ///
    /// # Errors
    ///
    /// This fails when the event could not be triggered.
    pub fn trigger_event(
        &self,
        event_node_id: &ua::NodeId,
        origin_id: &ua::NodeId,
        delete_event_node: bool,
    ) -> Result<ua::EventId> {
        // This out variable must be initialized without memory allocation because the call below
        // overwrites it in place, without releasing any held data first.
        let mut out_event_id = ua::ByteString::init();
        let status_code = ua::StatusCode::new(unsafe {
            UA_Server_triggerEvent(
                // SAFETY: Cast to `mut` pointer, function is marked `UA_THREADSAFE`.
                self.0.as_ptr().cast_mut(),
                // SAFETY: Passing as value is okay here, as the variables are only used for the
                // scope of the function and do not get modified.
                DataType::to_raw_copy(event_node_id),
                DataType::to_raw_copy(origin_id),
                out_event_id.as_mut_ptr(),
                delete_event_node,
            )
        });
        Error::verify_good(&status_code)?;
        let Some(event_id) = ua::EventId::new(out_event_id) else {
            return Err(Error::internal("trigger should return event ID"));
        };
        Ok(event_id)
    }

    /// Browses specific node.
    ///
    /// Use [`ua::BrowseDescription::default()`](ua::BrowseDescription) to set sensible defaults to
    /// browse a specific node's children (forward references of the `HierarchicalReferences` type)
    /// like this:
    ///
    /// ```
    /// # use open62541::{Result, Server, ua};
    /// use open62541_sys::UA_NS0ID_SERVER_SERVERSTATUS;
    ///
    /// # async fn example(server: &Server) -> Result<()> {
    /// let node_id = ua::NodeId::ns0(UA_NS0ID_SERVER_SERVERSTATUS);
    /// let browse_description = ua::BrowseDescription::default().with_node_id(&node_id);
    /// let (references, continuation_point) = server.browse(1000, &browse_description)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This fails when the node does not exist or it cannot be browsed.
    pub fn browse(
        &self,
        max_references: usize,
        browse_description: &ua::BrowseDescription,
    ) -> BrowseResult {
        let max_references = u32::try_from(max_references).map_err(|_| {
            Error::internal("maximum references to return should be in range of u32")
        })?;
        let result = unsafe {
            ua::BrowseResult::from_raw(UA_Server_browse(
                // SAFETY: Cast to `mut` pointer, function is marked `UA_THREADSAFE`.
                self.0.as_ptr().cast_mut(),
                max_references,
                browse_description.as_ptr(),
            ))
        };
        Error::verify_good(&result.status_code())?;
        to_browse_result(&result)
    }

    /// Browses continuation point for more references.
    ///
    /// This uses a continuation point returned from [`browse()`] whenever not all references were
    /// returned (due to `max_references`).
    ///
    /// # Errors
    ///
    /// This fails when the browsing was not successful.
    ///
    /// [`browse()`]: Self::browse
    pub fn browse_next(&self, continuation_point: &ua::ContinuationPoint) -> BrowseResult {
        let result = unsafe {
            ua::BrowseResult::from_raw(UA_Server_browseNext(
                // SAFETY: Cast to `mut` pointer, function is marked `UA_THREADSAFE`.
                self.0.as_ptr().cast_mut(),
                // We do not release the continuation point but browse it instead.
                false,
                continuation_point.as_byte_string().as_ptr(),
            ))
        };
        Error::verify_good(&result.status_code())?;
        to_browse_result(&result)
    }

    /// Browses nodes recursively.
    ///
    /// This is a non-standard version of the `Browse` service that recurses into child nodes. This
    /// handles possible loops (that can occur for non-hierarchical references) and adds every node
    /// at most once to the resulting list.
    ///
    /// Nodes are only added if they match the `NodeClassMask` in the `BrowseDescription`. However,
    /// child nodes are still recursed into if the `NodeClass` does not match. So it is possible,
    /// for example, to get all `VariableNode`s below a certain `ObjectNode`, with additional
    /// objects in the hierarchy below.
    ///
    /// # Errors
    ///
    /// This fails when the browsing was not successful.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::collections::HashSet;
    /// # use open62541::{DataType as _, ServerBuilder, ua};
    /// use open62541_sys::UA_NS0ID_SERVER_SERVERSTATUS;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let (server, _) = ServerBuilder::default().build();
    /// #
    /// let targets = server.browse_recursive(
    ///     &ua::BrowseDescription::default().with_node_id(
    ///         &ua::NodeId::ns0(UA_NS0ID_SERVER_SERVERSTATUS),
    ///     ),
    /// )?;
    ///
    /// // Browse above returns the expected number of well-known nodes.
    /// assert_eq!(targets.len(), 12);
    /// # assert_eq!(
    /// #     targets
    /// #         .as_slice()
    /// #         .iter()
    /// #         .map(|node| node.node_id().as_ns0().unwrap())
    /// #         .collect::<HashSet<_>>(),
    /// #     HashSet::from([
    /// #         open62541_sys::UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO,
    /// #         open62541_sys::UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_BUILDDATE,
    /// #         open62541_sys::UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_BUILDNUMBER,
    /// #         open62541_sys::UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_MANUFACTURERNAME,
    /// #         open62541_sys::UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_PRODUCTNAME,
    /// #         open62541_sys::UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_PRODUCTURI,
    /// #         open62541_sys::UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_SOFTWAREVERSION,
    /// #         open62541_sys::UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME,
    /// #         open62541_sys::UA_NS0ID_SERVER_SERVERSTATUS_SECONDSTILLSHUTDOWN,
    /// #         open62541_sys::UA_NS0ID_SERVER_SERVERSTATUS_SHUTDOWNREASON,
    /// #         open62541_sys::UA_NS0ID_SERVER_SERVERSTATUS_STARTTIME,
    /// #         open62541_sys::UA_NS0ID_SERVER_SERVERSTATUS_STATE,
    /// #     ])
    /// # );
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn browse_recursive(
        &self,
        browse_description: &ua::BrowseDescription,
    ) -> Result<ua::Array<ua::ExpandedNodeId>> {
        let mut result_size = 0;
        let mut result_ptr = ptr::null_mut();
        let status_code = ua::StatusCode::new(unsafe {
            UA_Server_browseRecursive(
                // SAFETY: Cast to `mut` pointer, function is marked `UA_THREADSAFE`.
                self.0.as_ptr().cast_mut(),
                browse_description.as_ptr(),
                &mut result_size,
                &mut result_ptr,
            )
        });
        Error::verify_good(&status_code)?;
        let Some(result) = ua::Array::from_raw_parts(result_size, result_ptr) else {
            return Err(Error::internal("recursive browse should return result"));
        };
        Ok(result)
    }

    /// Browses simplified browse path.
    ///
    /// This specifies a relative path using [`ua::QualifiedName`] instead of [`ua::RelativePath`],
    /// using forward references and subtypes of `HierarchicalReferences`, matching the defaults of
    /// [`ua::BrowseDescription`]. All nodes followed by `browse_path` shall be of the node classes
    /// `Object` or `Variable`.
    ///
    /// See [`translate_browse_path_to_node_ids()`](Self::translate_browse_path_to_node_ids) if you
    /// need more control over the references involved.
    ///
    /// # Errors
    ///
    /// This fails when the browsing was not successful.
    ///
    /// # Examples
    ///
    /// ```
    /// # use open62541::{DataType as _, ServerBuilder, ua};
    /// use open62541_sys::{
    ///     UA_NS0ID_SERVER_SERVERSTATUS, UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_PRODUCTNAME,
    /// };
    ///
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let (server, _) = ServerBuilder::default().build();
    /// #
    /// let target_name_1 = ua::QualifiedName::new(0, "BuildInfo");
    /// let target_name_2 = ua::QualifiedName::new(0, "ProductName");
    ///
    /// let targets = server.browse_simplified_browse_path(
    ///     &ua::NodeId::ns0(UA_NS0ID_SERVER_SERVERSTATUS),
    ///     &[target_name_1, target_name_2],
    /// )?;
    ///
    /// // Translation above returns a single target.
    /// assert_eq!(targets.len(), 1);
    /// let target = &targets[0];
    ///
    /// // The given path leads to the right node ID.
    /// assert_eq!(
    ///     target.target_id(),
    ///     &ua::NodeId::ns0(UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_PRODUCTNAME)
    ///         .into_expanded_node_id()
    /// );
    ///
    /// // All relative path elements were processed.
    /// assert_eq!(target.remaining_path_index(), None);
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn browse_simplified_browse_path(
        &self,
        origin: &ua::NodeId,
        browse_path: &[ua::QualifiedName],
    ) -> Result<ua::Array<ua::BrowsePathTarget>> {
        // SAFETY: The raw pointer is only used in the call below and `browse_path` is still alive
        // until the end of this function.
        let (browse_path_size, browse_path_ptr) =
            unsafe { ua::Array::raw_parts_from_slice(browse_path) };
        let result = unsafe {
            ua::BrowsePathResult::from_raw(UA_Server_browseSimplifiedBrowsePath(
                // SAFETY: Cast to `mut` pointer, function is marked `UA_THREADSAFE`.
                self.0.as_ptr().cast_mut(),
                // SAFETY: The function expects a copy but does not take ownership. In particular,
                // memory lives only on the stack and is not released when the function returns.
                DataType::to_raw_copy(origin),
                browse_path_size,
                browse_path_ptr,
            ))
        };
        Error::verify_good(&result.status_code())?;
        let targets = result
            .targets()
            .ok_or(Error::internal("browse should return targets"))?;
        Ok(targets)
    }

    /// Translates browse path to node IDs.
    ///
    /// # Errors
    ///
    /// An error will be returned if the translation was not successful.
    ///
    /// # Examples
    ///
    /// ```
    /// # use open62541::{DataType as _, ServerBuilder, ua};
    /// use open62541_sys::{
    ///     UA_NS0ID_SERVER_SERVERSTATUS, UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_PRODUCTNAME,
    /// };
    ///
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let (server, _) = ServerBuilder::default().build();
    /// #
    /// let target_name_1 = ua::QualifiedName::new(0, "BuildInfo");
    /// let target_name_2 = ua::QualifiedName::new(0, "ProductName");
    ///
    /// let targets = server.translate_browse_path_to_node_ids(&ua::BrowsePath::init()
    ///     .with_starting_node(&ua::NodeId::ns0(UA_NS0ID_SERVER_SERVERSTATUS))
    ///     .with_relative_path(&ua::RelativePath::init()
    ///         .with_elements(&[
    ///             ua::RelativePathElement::init().with_target_name(&target_name_1),
    ///             ua::RelativePathElement::init().with_target_name(&target_name_2),
    ///         ])
    ///     )
    /// )?;
    ///
    /// // Translation above returns a single target.
    /// assert_eq!(targets.len(), 1);
    /// let target = &targets[0];
    ///
    /// // The given path leads to the right node ID.
    /// assert_eq!(
    ///     target.target_id(),
    ///     &ua::NodeId::ns0(UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_PRODUCTNAME)
    ///         .into_expanded_node_id()
    /// );
    ///
    /// // All relative path elements were processed.
    /// assert_eq!(target.remaining_path_index(), None);
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn translate_browse_path_to_node_ids(
        &self,
        browse_path: &ua::BrowsePath,
    ) -> Result<ua::Array<ua::BrowsePathTarget>> {
        let result = unsafe {
            ua::BrowsePathResult::from_raw(UA_Server_translateBrowsePathToNodeIds(
                // SAFETY: Cast to `mut` pointer, function is marked `UA_THREADSAFE`.
                self.0.as_ptr().cast_mut(),
                browse_path.as_ptr(),
            ))
        };
        Error::verify_good(&result.status_code())?;
        let targets = result
            .targets()
            .ok_or(Error::internal("translation should return targets"))?;
        Ok(targets)
    }

    /// Reads node attribute.
    ///
    /// This method supports static dispatch to the correct value type at compile time and can be
    /// used in two ways:
    ///
    /// 1. Use statically known attribute type `_T` from [`ua::AttributeId`] to get correct value
    ///    type directly.
    /// 2. Use dynamic [`ua::AttributeId`] value and handle the resulting [`ua::Variant`].
    ///
    /// # Errors
    ///
    /// This fails when the node does not exist or the attribute cannot be read.
    ///
    /// # Examples
    ///
    /// ```
    /// # use open62541::{DataType as _, ServerBuilder, ua};
    /// # use open62541_sys::UA_NS0ID_SERVER_SERVERSTATUS;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let (server, _) = ServerBuilder::default().build();
    /// #
    /// let node_id = ua::NodeId::ns0(UA_NS0ID_SERVER_SERVERSTATUS);
    ///
    /// // Use static dispatch to get expected value type directly:
    /// let browse_name = server
    ///     .read_attribute(&node_id, ua::AttributeId::BROWSENAME_T)?
    ///     .into_value();
    /// // The type of `browse_name` is `ua::QualifiedName` here.
    /// assert_eq!(browse_name, ua::QualifiedName::new(0, "ServerStatus"));
    ///
    /// // Use dynamic attribute and unwrap `ua::Variant` manually:
    /// let attribute_id: ua::AttributeId = ua::AttributeId::BROWSENAME;
    /// let browse_name = server.read_attribute(&node_id, &attribute_id)?.into_value();
    /// // The type of `browse_name` is `ua::Variant` here.
    /// let browse_name = browse_name.to_scalar::<ua::QualifiedName>().unwrap();
    /// assert_eq!(browse_name, ua::QualifiedName::new(0, "ServerStatus"));
    /// #
    /// # let unknown_node_id = ua::NodeId::ns0(123_456_789);
    /// # assert!(server.read_attribute(&unknown_node_id, ua::AttributeId::NODEID_T).is_err());
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn read_attribute<T: Attribute>(
        &self,
        node_id: &ua::NodeId,
        attribute: T,
    ) -> Result<DataValue<T::Value>> {
        let item = ua::ReadValueId::init()
            .with_node_id(node_id)
            .with_attribute_id(&attribute.id());
        let result = unsafe {
            ua::DataValue::from_raw(UA_Server_read(
                self.0.as_ptr().cast_mut(),
                item.as_ptr(),
                ua::TimestampsToReturn::NEITHER.into_raw(),
            ))
        };
        result.to_generic::<T::Value>()
    }

    /// Writes node value.
    ///
    /// # Errors
    ///
    /// This fails when the node does not exist or its value attribute cannot be written.
    pub fn write_value(&self, node_id: &ua::NodeId, value: &ua::Variant) -> Result<()> {
        let status_code = ua::StatusCode::new(unsafe {
            __UA_Server_write(
                // SAFETY: Cast to `mut` pointer, function is marked `UA_THREADSAFE`.
                self.0.as_ptr().cast_mut(),
                node_id.as_ptr(),
                // Passing ownership is trivial with primitive value (`u32`).
                ua::AttributeId::VALUE.into_raw(),
                ua::Variant::data_type(),
                value.as_ptr().cast::<c_void>(),
            )
        });
        Error::verify_good(&status_code)
    }

    /// Reads object property.
    ///
    /// # Errors
    ///
    /// This fails when reading the object property was not successful.
    ///
    /// # Examples
    ///
    /// ```
    /// # use open62541::{DataType as _, Node, ServerBuilder, ua};
    /// # use open62541_sys::{
    /// #     UA_NS0ID_HASPROPERTY, UA_NS0ID_OBJECTSFOLDER, UA_NS0ID_ORGANIZES, UA_NS0ID_STRING,
    /// # };
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let (server, _) = ServerBuilder::default().build();
    /// #
    /// # let object_node_id = server.add_node(Node::new(
    /// #     ua::NodeId::ns0(UA_NS0ID_OBJECTSFOLDER),
    /// #     ua::NodeId::ns0(UA_NS0ID_ORGANIZES),
    /// #     ua::QualifiedName::new(1, "SomeObject"),
    /// #     ua::ObjectAttributes::init(),
    /// # ))?;
    /// # let variable_node_id = server.add_node(Node::new(
    /// #     object_node_id.clone(),
    /// #     ua::NodeId::ns0(UA_NS0ID_HASPROPERTY),
    /// #     ua::QualifiedName::new(1, "SomeVariable"),
    /// #     ua::VariableAttributes::init()
    /// #         .with_data_type(&ua::NodeId::ns0(UA_NS0ID_STRING))
    /// #         .with_value_rank(-1),
    /// # ))?;
    /// #
    /// # server.write_object_property(
    /// #     &object_node_id,
    /// #     &ua::QualifiedName::new(1, "SomeVariable"),
    /// #     &ua::Variant::scalar(ua::String::new("LoremIpsum")?),
    /// # )?;
    /// #
    /// let value = server.read_object_property(
    ///     &object_node_id,
    ///     &ua::QualifiedName::new(1, "SomeVariable"),
    /// )?;
    /// # assert_eq!(
    /// #     value.as_scalar::<ua::String>().and_then(ua::String::as_str),
    /// #     Some("LoremIpsum")
    /// # );
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn read_object_property(
        &self,
        object_id: &ua::NodeId,
        property_name: &ua::QualifiedName,
    ) -> Result<ua::Variant> {
        let mut value = ua::Variant::init();
        let status_code = unsafe {
            ua::StatusCode::new(UA_Server_readObjectProperty(
                // SAFETY: Cast to `mut` pointer, function is marked `UA_THREADSAFE`.
                self.0.as_ptr().cast_mut(),
                // SAFETY: The function expects copies but does not take ownership. In particular,
                // memory lives only on the stack and is not released when the function returns.
                DataType::to_raw_copy(object_id),
                DataType::to_raw_copy(property_name),
                value.as_mut_ptr(),
            ))
        };
        Error::verify_good(&status_code)?;
        Ok(value)
    }

    /// Writes object property.
    ///
    /// The property is represented as a `VariableNode` with a `HasProperty` reference from the
    /// `ObjectNode`. The `VariableNode` is identified by its `BrowseName`. Writing the property
    /// sets the value attribute of the `VariableNode`.
    ///
    /// # Errors
    ///
    /// This fails when writing the object property was not successful.
    ///
    /// # Examples
    ///
    /// ```
    /// # use open62541::{DataType as _, Node, ServerBuilder, ua};
    /// # use open62541_sys::{
    /// #     UA_NS0ID_HASPROPERTY, UA_NS0ID_OBJECTSFOLDER, UA_NS0ID_ORGANIZES, UA_NS0ID_STRING,
    /// # };
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let (server, _) = ServerBuilder::default().build();
    /// #
    /// # let object_node_id = server.add_node(Node::new(
    /// #     ua::NodeId::ns0(UA_NS0ID_OBJECTSFOLDER),
    /// #     ua::NodeId::ns0(UA_NS0ID_ORGANIZES),
    /// #     ua::QualifiedName::new(1, "SomeObject"),
    /// #     ua::ObjectAttributes::init(),
    /// # ))?;
    /// # let variable_node_id = server.add_node(Node::new(
    /// #     object_node_id.clone(),
    /// #     ua::NodeId::ns0(UA_NS0ID_HASPROPERTY),
    /// #     ua::QualifiedName::new(1, "SomeVariable"),
    /// #     ua::VariableAttributes::init()
    /// #         .with_data_type(&ua::NodeId::ns0(UA_NS0ID_STRING))
    /// #         .with_value_rank(-1),
    /// # ))?;
    /// #
    /// server.write_object_property(
    ///     &object_node_id,
    ///     &ua::QualifiedName::new(1, "SomeVariable"),
    ///     &ua::Variant::scalar(ua::String::new("LoremIpsum")?),
    /// )?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn write_object_property(
        &self,
        object_id: &ua::NodeId,
        property_name: &ua::QualifiedName,
        value: &ua::Variant,
    ) -> Result<()> {
        let status_code = unsafe {
            ua::StatusCode::new(UA_Server_writeObjectProperty(
                // SAFETY: Cast to `mut` pointer, function is marked `UA_THREADSAFE`.
                self.0.as_ptr().cast_mut(),
                // SAFETY: The function expects copies but does not take ownership. In particular,
                // memory lives only on the stack and is not released when the function returns.
                DataType::to_raw_copy(object_id),
                DataType::to_raw_copy(property_name),
                DataType::to_raw_copy(value),
            ))
        };
        Error::verify_good(&status_code)
    }
}

#[derive(Debug)]
pub struct ServerRunner {
    server: Arc<ua::Server>,

    /// [`AccessControl`] instances may hold additional data that must be kept alive until server is
    /// shut down. The sentinel value cleans this up when it is dropped.
    access_control_sentinel: Option<Box<dyn Any + Send>>,
}

impl ServerRunner {
    #[must_use]
    fn new(server: &Arc<ua::Server>, access_control_sentinel: Option<Box<dyn Any + Send>>) -> Self {
        Self {
            server: Arc::clone(server),
            access_control_sentinel,
        }
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
        let Self {
            server,
            access_control_sentinel,
        } = self;

        let status_code = ua::StatusCode::new(unsafe {
            UA_Server_runUntilInterrupt(
                // SAFETY: Cast to `mut` pointer. Function is not marked `UA_THREADSAFE` but we make
                // sure that it can only be invoked a single time (ownership of `ServerRunner`). The
                // examples in `open62541` demonstrate that running the server in its own thread and
                // interacting with it as we do through `Server` is okay.
                server.as_ptr().cast_mut(),
            )
        });
        Error::verify_good(&status_code)?;

        // Only when the server has finished shutting down, we are allowed to drop sentinel values.
        drop(access_control_sentinel);

        Ok(())
    }

    /// Runs the server until it is cancelled.
    ///
    /// The server is shut down cleanly when `is_cancelled` returns true at which point the method
    /// returns.
    ///
    /// # Errors
    ///
    /// This fails when the server cannot be started.
    pub fn run_until_cancelled(self, is_cancelled: &mut impl FnMut() -> bool) -> Result<()> {
        let Self {
            server,
            access_control_sentinel,
        } = self;

        log::info!("Starting up server");

        let status_code = ua::StatusCode::new(unsafe {
            // The prologue part of `UA_Server_run()`.
            open62541_sys::UA_Server_run_startup(
                // SAFETY: Cast to `mut` pointer. Function is not marked `UA_THREADSAFE` but we make
                // sure that it can only be invoked a single time (ownership of `ServerRunner`). The
                // examples in `open62541` demonstrate that running the server in its own thread and
                // interacting with it as we do through `Server` is okay.
                server.as_ptr().cast_mut(),
            )
        });
        Error::verify_good(&status_code)?;

        while !is_cancelled() {
            // Track time of iteration start to report iteration times below.
            let start_of_iteration = Instant::now();

            log::trace!("Running iterate");

            unsafe {
                // Execute a single iteration of the server's main loop.
                //
                // We discard the returned value, i.e. how long we can wait until the next scheduled
                // callback, as `UA_Server_run_iterate()` does the required waiting. See
                // <https://github.com/open62541/open62541/blob/d4c5aaa2a755d846d8517f96995d318a66742d42/include/open62541/server.h#L474-L483>
                // for more information.
                let _ = open62541_sys::UA_Server_run_iterate(
                    // SAFETY: Cast to `mut` pointer. This is safe despite missing `UA_THREADSAFE`.
                    server.as_ptr().cast_mut(),
                    true,
                );
            }

            let time_taken = start_of_iteration.elapsed();
            log::trace!("Iterate run took {time_taken:?}");
        }

        log::info!("Shutting down cancelled server");

        let status_code = ua::StatusCode::new(unsafe {
            // The epilogue part of `UA_Server_run()`.
            open62541_sys::UA_Server_run_shutdown(
                // SAFETY: Cast to `mut` pointer. This is safe despite missing `UA_THREADSAFE`.
                server.as_ptr().cast_mut(),
            )
        });
        if let Err(error) = Error::verify_good(&status_code) {
            // Unexpected error.
            log::error!("Shutdown of cancelled server failed with {error}");
            // We do not forward the error to the caller because it happened during shutdown. Errors
            // during startup are handled and returned above.
            return Ok(());
        }

        // Only when the server has finished shutting down, we are allowed to drop sentinel values.
        drop(access_control_sentinel);

        Ok(())
    }
}

/// Converts [`ua::BrowseResult`] to our public result type.
fn to_browse_result(result: &ua::BrowseResult) -> BrowseResult {
    // Make sure to verify the inner status code inside `BrowseResult`. The service request finishes
    // without error, even when browsing the node has failed.
    Error::verify_good(&result.status_code())?;

    let Some(references) = result.references() else {
        return Err(Error::internal("browse should return references"));
    };

    Ok((references.into_vec(), result.continuation_point()))
}
