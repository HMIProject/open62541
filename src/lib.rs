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
mod monitored_item;
mod subscription;
pub mod ua;

#[cfg(feature = "tokio")]
pub use self::{
    async_client::AsyncClient, async_monitored_item::AsyncMonitoredItem,
    async_subscription::AsyncSubscription,
};
pub(crate) use self::{
    callback::CallbackOnce,
    data_type::{data_type, DataType},
};
pub use self::{
    client::{Client, ClientBuilder},
    error::Error,
    monitored_item::MonitoredItem,
    subscription::Subscription,
};

#[derive(Clone, Copy, Debug)]
pub struct SubscriptionId(u32);

#[derive(Clone, Copy, Debug)]
pub struct MonitoredItemId(u32);
