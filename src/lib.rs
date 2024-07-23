//! Rust wrapper for the [open62541](https://www.open62541.org) library.
//!
//! # Examples
//!
//! ## Connect to server
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
//! ## Read node's value attribute
//!
//! ```no_run
//! # use open62541::AsyncClient;
//! use open62541::ua::NodeId;
//!
//! # #[tokio::main]
//! # async fn main() -> anyhow::Result<()> {
//! #
//! # let client = AsyncClient::new("opc.tcp://opcuademo.sterfive.com:26543")?;
//! #
//! let node_id = NodeId::numeric(0, 2258); // Server/ServerStatus/CurrentTime
//!
//! let value = client.read_value(&node_id).await?;
//!
//! println!("Received value: {value:?}");
//! #
//! # Ok(())
//! # }
//! ```
//!
//! ## Watch node for changes in value attribute
//!
//! ```no_run
//! # use open62541::{AsyncClient, ua::NodeId};
//! #
//! # #[tokio::main]
//! # async fn main() -> anyhow::Result<()> {
//! #
//! # let client = AsyncClient::new("opc.tcp://opcuademo.sterfive.com:26543")?;
//! #
//! # let node_id = NodeId::numeric(0, 2258); // Server/ServerStatus/CurrentTime
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

mod client;
mod data_type;
mod error;
mod server;
mod service;
pub mod ua;

#[cfg(feature = "tokio")]
mod async_client;
#[cfg(feature = "tokio")]
mod async_monitored_item;
#[cfg(feature = "tokio")]
mod async_subscription;
#[cfg(feature = "tokio")]
mod callback;
mod logger;
mod traits;
mod userdata;
mod value;

pub use self::{
    client::{Client, ClientBuilder},
    data_type::DataType,
    error::{Error, Result},
    server::{
        DataSource, DataSourceError, DataSourceReadContext, DataSourceResult,
        DataSourceWriteContext, ObjectNode, Server, ServerBuilder, ServerRunner, VariableNode,
    },
    traits::Attributes,
    userdata::Userdata,
    value::{ScalarValue, ValueType, VariantValue},
};
pub(crate) use self::{
    data_type::{bitmask_ops, data_type, enum_variants},
    logger::logger,
    service::{ServiceRequest, ServiceResponse},
    value::{ArrayValue, NonScalarValue},
};

#[cfg(feature = "tokio")]
pub use self::{
    async_client::AsyncClient,
    async_monitored_item::AsyncMonitoredItem,
    async_subscription::AsyncSubscription,
    callback::{CallbackOnce, CallbackStream},
};
