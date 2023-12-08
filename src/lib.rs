//! Rust wrapper for the [open62541](https://www.open62541.org) library.
//!
//! # Examples
//!
//! ## Connect to server
//!
//! ```
//! # use std::pin::pin;
//! # use futures::StreamExt;
//! use open62541::Client;
//!
//! # #[tokio::main(flavor = "current_thread")]
//! # async fn main() -> anyhow::Result<()> {
//! #
//! let client = Client::new("opc.tcp://opcuademo.sterfive.com:26543")?.into_async();
//! #
//! # Ok(())
//! # }
//! ```
//!
//! ## Read node's value attribute
//!
//! ```
//! # use std::pin::pin;
//! # use futures::StreamExt;
//! use open62541::ua::NodeId;
//! # use open62541::Client;
//!
//! # #[tokio::main(flavor = "current_thread")]
//! # async fn main() -> anyhow::Result<()> {
//! #
//! # let client = Client::new("opc.tcp://opcuademo.sterfive.com:26543")?.into_async();
//! #
//! let node_id = NodeId::new_numeric(0, 2258); // Server/ServerStatus/CurrentTime
//!
//! let value = client.read_value(node_id).await?;
//!
//! println!("Received value: {value:?}");
//! #
//! # Ok(())
//! # }
//! ```
//!
//! ## Watch node for changes in value attribute
//!
//! ```
//! # use std::pin::pin;
//! # use futures::{stream::empty, StreamExt};
//! # use open62541::{Client, ua::{DataValue, NodeId}};
//!
//! # #[tokio::main(flavor = "current_thread")]
//! # async fn main() -> anyhow::Result<()> {
//! #
//! # let client = Client::new("opc.tcp://opcuademo.sterfive.com:26543")?.into_async();
//! #
//! # let node_id = NodeId::new_numeric(0, 2258); // Server/ServerStatus/CurrentTime
//! #
//! // Get stream that contains value updates from the server.
//! let value_stream = client.value_stream(node_id).await?;
//! # let value_stream = empty::<DataValue>();
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
mod callback;
mod client;
mod data_type;
mod error;
pub mod ua;

#[cfg(feature = "tokio")]
pub use self::{
    async_client::AsyncClient, async_monitored_item::AsyncMonitoredItem,
    async_subscription::AsyncSubscription,
};
pub(crate) use self::{
    callback::{CallbackMut, CallbackOnce},
    data_type::{data_type, DataType},
};
pub use self::{
    client::{Client, ClientBuilder},
    error::Error,
};

/// ID of subscription.
///
/// This holds an ID to track subscriptions in an OPC UA connection.
#[derive(Clone, Copy, Debug)]
pub struct SubscriptionId(u32);

/// ID of monitored item.
///
/// This holds an ID to track monitored items in an OPC UA connection.
#[derive(Clone, Copy, Debug)]
pub struct MonitoredItemId(u32);
