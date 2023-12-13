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
//! let node_id = NodeId::new_numeric(0, 2258); // Server/ServerStatus/CurrentTime
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
//! use futures::StreamExt;
//! use std::pin::pin;
//!
//! use open62541::{AsyncClient, ua::NodeId};
//!
//! # #[tokio::main(flavor = "current_thread")]
//! # async fn main() -> anyhow::Result<()> {
//! #
//! # let client = AsyncClient::new("opc.tcp://opcuademo.sterfive.com:26543")?;
//! #
//! # let node_id = NodeId::new_numeric(0, 2258); // Server/ServerStatus/CurrentTime
//! #
//! // Get stream with value updates from the server.
//! let value_stream = client.value_stream(&node_id).await?;
//! // Pinning is required to consume stream items.
//! let mut pinned_stream = pin!(value_stream);
//!
//! while let Some(value) = pinned_stream.next().await {
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
pub(crate) use self::callback::{CallbackOnce, CallbackStream};
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
