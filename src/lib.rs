//! Rust wrapper for the [open62541](https://www.open62541.org) library.
//!
//! # Examples
//!
//! ## Client: Connect to server
//!
//! Use [`AsyncClient`] to asynchronously connect to an OPC UA server:
//!
//! ```no_run
//! use open62541::AsyncClient;
//!
//! # #[tokio::main]
//! # async fn main() -> anyhow::Result<()> {
//! #
//! let client = AsyncClient::new("opc.tcp://opcuademo.sterfive.com:26543")?;
//! #
//! # Ok(())
//! # }
//! ```
//!
//! This requires an async runtime such as [`tokio`].
//!
//! ## Client: Read node's value attribute
//!
//! Read a variable node's value attribute with [`AsyncClient::read_value()`]:
//!
//! ```no_run
//! # use open62541::AsyncClient;
//! use open62541::ua;
//!
//! # #[tokio::main]
//! # async fn main() -> anyhow::Result<()> {
//! #
//! # let client = AsyncClient::new("opc.tcp://opcuademo.sterfive.com:26543")?;
//! #
//! let node_id = ua::NodeId::numeric(0, 2258); // Server/ServerStatus/CurrentTime
//!
//! let value = client.read_value(&node_id).await?;
//!
//! println!("Received value: {value:?}");
//! #
//! # Ok(())
//! # }
//! ```
//!
//! Use [`AsyncClient::read_attribute()`] and related methods to read other attributes other than
//! the value.
//!
//! ## Client: Watch node for changes in value attribute
//!
//! Subscribe to a node's value by creating a subscription with
//! [`AsyncClient::create_subscription()`] and adding monitored items to it with
//! [`AsyncSubscription::create_monitored_item()`]:
//!
//! ```no_run
//! # use open62541::{AsyncClient, ua};
//! #
//! # #[tokio::main]
//! # async fn main() -> anyhow::Result<()> {
//! #
//! # let client = AsyncClient::new("opc.tcp://opcuademo.sterfive.com:26543")?;
//! #
//! # let node_id = ua::NodeId::numeric(0, 2258); // Server/ServerStatus/CurrentTime
//! #
//! // Create subscription that receives the updates.
//! let subscription = client.create_subscription().await?;
//! // Create monitored item to receive node updates.
//! let mut monitored_item = subscription.create_monitored_item(&node_id).await?;
//!
//! while let Some(value) = monitored_item.next().await {
//!     println!("Received value: {value:?}");
//! }
//! #
//! # Ok(())
//! # }
//! ```
//!
//! ## Server: Run server
//!
//! Create an OPC UA server with [`Server::new()`]. When instantiating a server, you receive a
//! [`ServerRunner`] along with the [`Server`]. Use it to run the server until interrupted:
//!
//! ```no_run
//! use open62541::Server;
//!
//! # #[tokio::main]
//! # async fn main() -> anyhow::Result<()> {
//! let (server, runner) = Server::new();
//!
//! // Define data nodes on `server`.
//!
//! runner.run_until_interrupt()?;
//! #
//! # Ok(())
//! # }
//! ```
//!
//! By default, [`ServerRunner::run()`] runs on the current thread. Use
//! [`thread::spawn()`](std::thread::spawn) to run it in a different thread.
//!
//! ## Server: Define object and managed variable nodes
//!
//! Define nodes with [`Server::add_object_node()`] and related methods:
//!
//! ```
//! # use open62541::Server;
//! use open62541::{ObjectNode, ua, VariableNode};
//! use open62541_sys::{
//!     UA_NS0ID_BASEDATAVARIABLETYPE, UA_NS0ID_FOLDERTYPE, UA_NS0ID_OBJECTSFOLDER,
//!     UA_NS0ID_ORGANIZES, UA_NS0ID_STRING,
//! };
//!
//! # #[tokio::main]
//! # async fn main() -> anyhow::Result<()> {
//! # let (server, runner) = Server::new();
//! #
//! let object_node_id = server.add_object_node(ObjectNode {
//!     requested_new_node_id: None,
//!     parent_node_id: ua::NodeId::ns0(UA_NS0ID_OBJECTSFOLDER),
//!     reference_type_id: ua::NodeId::ns0(UA_NS0ID_ORGANIZES),
//!     browse_name: ua::QualifiedName::new(1, "SomeFolder"),
//!     type_definition: ua::NodeId::ns0(UA_NS0ID_FOLDERTYPE),
//!     attributes: ua::ObjectAttributes::default(),
//! })?;
//!
//! let variable_node_id = server.add_variable_node(VariableNode {
//!     requested_new_node_id: None,
//!     parent_node_id: object_node_id,
//!     reference_type_id: ua::NodeId::ns0(UA_NS0ID_ORGANIZES),
//!     browse_name: ua::QualifiedName::new(1, "SomeVariable"),
//!     type_definition: ua::NodeId::ns0(UA_NS0ID_BASEDATAVARIABLETYPE),
//!     attributes: ua::VariableAttributes::default()
//!         .with_data_type(&ua::NodeId::ns0(UA_NS0ID_STRING)),
//! })?;
//!
//! server.write_value(
//!     &variable_node_id,
//!     &ua::Variant::scalar(ua::String::new("Lorem Ipsum")?),
//! )?;
//! #
//! # Ok(())
//! # }
//! ```
//!
//! Nodes may also be added (and removed) while the server is running.
//!
//! ## Server: Define data source variable nodes (callback-driven)
//!
//! Implement [`DataSource`] for custom types to enable callback-driven read and write access
//! through OPC UA variables.
//!
//! Use [`DataSourceReadContext`] and [`DataSourceWriteContext`] to set the value to return to the
//! client, or access the incoming value received from the client:
//!
//! ```
//! # use open62541::{ObjectNode, Server, ua, VariableNode};
//! use open62541::{DataSource, DataSourceReadContext, DataSourceResult, DataSourceWriteContext};
//! # use open62541_sys::{
//! #     UA_NS0ID_BASEDATAVARIABLETYPE, UA_NS0ID_OBJECTSFOLDER, UA_NS0ID_ORGANIZES,
//! #     UA_NS0ID_STRING,
//! # };
//!
//! struct SomeDataSource {
//!     some_value: u32,
//! }
//!
//! impl DataSource for SomeDataSource {
//!     fn read(&mut self, ctx: &mut DataSourceReadContext) -> DataSourceResult {
//!         let value = format!("This is #{value}", value = self.some_value);
//!         ctx.set_variant(ua::Variant::scalar(ua::String::new(&value)?));
//!         Ok(())
//!     }
//!
//!     fn write(&mut self, ctx: &mut DataSourceWriteContext) -> DataSourceResult {
//!         println!("Received value: {value:?}", value = ctx.value());
//!         self.some_value += 1;
//!         Ok(())
//!     }
//! }
//!
//! # #[tokio::main]
//! # async fn main() -> anyhow::Result<()> {
//! # let (server, runner) = Server::new();
//! #
//! # let object_node_id = ua::NodeId::ns0(UA_NS0ID_OBJECTSFOLDER);
//! #
//! let variable_node = VariableNode {
//!     requested_new_node_id: None,
//!     parent_node_id: object_node_id,
//!     reference_type_id: ua::NodeId::ns0(UA_NS0ID_ORGANIZES),
//!     browse_name: ua::QualifiedName::new(1, "SomeVariable"),
//!     type_definition: ua::NodeId::ns0(UA_NS0ID_BASEDATAVARIABLETYPE),
//!     attributes: ua::VariableAttributes::default()
//!         .with_data_type(&ua::NodeId::ns0(UA_NS0ID_STRING))
//!         .with_access_level(
//!             &ua::AccessLevelType::NONE
//!                 .with_current_read(true)
//!                 .with_current_write(true),
//!         ),
//! };
//!
//! let variable_node_id = server.add_data_source_variable_node(
//!     variable_node,
//!     SomeDataSource { some_value: 0 },
//! )?;
//! #
//! # Ok(())
//! # }
//! ```

#[cfg(feature = "tokio")]
mod async_client;
#[cfg(feature = "tokio")]
mod async_monitored_item;
#[cfg(feature = "tokio")]
mod async_subscription;
mod attributes;
mod browse_result;
mod callback;
#[cfg(feature = "tokio")]
mod callback_stream;
mod client;
mod data_type;
mod data_value;
mod error;
mod server;
mod service;
#[cfg(feature = "mbedtls")]
mod ssl;
mod traits;
pub mod ua;
mod userdata;
mod value;

#[cfg(feature = "tokio")]
pub use self::{
    async_client::AsyncClient,
    async_monitored_item::{
        AsyncMonitoredItem, MonitoredItemAttribute, MonitoredItemBuilder, MonitoredItemKind,
        MonitoredItemValue,
    },
    async_subscription::{AsyncSubscription, SubscriptionBuilder},
    callback_stream::CallbackStream,
};
pub use self::{
    browse_result::BrowseResult,
    callback::CallbackOnce,
    client::{Client, ClientBuilder},
    data_type::DataType,
    data_value::DataValue,
    error::{Error, Result},
    server::{
        AccessControl, DataSource, DataSourceError, DataSourceReadContext, DataSourceResult,
        DataSourceWriteContext, DefaultAccessControl, DefaultAccessControlWithLoginCallback,
        MethodCallback, MethodCallbackContext, MethodCallbackError, MethodCallbackResult,
        MethodNode, Node, ObjectNode, Server, ServerBuilder, ServerRunner, VariableNode,
    },
    service::{ServiceRequest, ServiceResponse},
    traits::{
        Attribute, Attributes, CustomCertificateVerification, DataTypeExt, FilterOperand,
        MonitoringFilter,
    },
    userdata::{Userdata, UserdataSentinel},
    value::{ScalarValue, ValueType, VariantValue},
};
pub(crate) use self::{
    client::ClientContext,
    data_type::{bitmask_ops, data_type, enum_variants},
    value::{ArrayValue, NonScalarValue},
};
#[cfg(feature = "mbedtls")]
pub use self::{
    ssl::{create_certificate, Certificate, Password, PrivateKey},
    traits::PrivateKeyPasswordCallback,
};

/// IANA-assigned OPC UA port number.
pub const DEFAULT_PORT_NUMBER: u16 = 4840;
