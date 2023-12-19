//! Rust wrapper for the [open62541](https://www.open62541.org) library.
//!
//! # Examples
//!
//! ## Connect to server
//!
//! ```no_run
//! use open62541::AsyncClient;
//!
//! # #[tokio::main(flavor = "current_thread")]
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
//! use open62541::{AsyncClient, ua::NodeId};
//!
//! # #[tokio::main(flavor = "current_thread")]
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
//! use futures::StreamExt as _;
//!
//! use open62541::{AsyncClient, ua::NodeId};
//!
//! # #[tokio::main(flavor = "current_thread")]
//! # async fn main() -> anyhow::Result<()> {
//! #
//! # let client = AsyncClient::new("opc.tcp://opcuademo.sterfive.com:26543")?;
//! #
//! # let node_id = NodeId::numeric(0, 2258); // Server/ServerStatus/CurrentTime
//! #
//! // Create subscription that receives the updates.
//! let subscription = client.create_subscription().await?;
//! // Create monitored item to create  node updates.
//! let mut monitored_item = subscription.create_monitored_item(&node_id).await?;
//!
//! while let Some(value) = monitored_item.next().await {
//!     println!("Received value: {value:?}");
//! }
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
#[cfg(feature = "tokio")]
mod callback;
mod client;
mod data_type;
mod error;
mod service;
pub mod ua;

#[cfg(feature = "tokio")]
#[doc(hidden)]
pub use self::callback::{CallbackOnce, CallbackStream};
#[cfg(feature = "tokio")]
pub use self::{
    async_client::AsyncClient, async_monitored_item::AsyncMonitoredItem,
    async_subscription::AsyncSubscription,
};
pub use self::{
    client::{Client, ClientBuilder},
    error::Error,
};
pub(crate) use self::{
    data_type::{data_type, DataType},
    service::{ServiceRequest, ServiceResponse},
};
