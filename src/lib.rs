mod client;
mod data_type;
mod error;
mod monitored_item;
mod subscription;
pub mod ua;

pub(crate) use self::data_type::{data_type, DataType};
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
