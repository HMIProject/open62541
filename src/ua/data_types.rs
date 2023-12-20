//! Thin wrappers for OPC UA data types from [`open62541_sys`].

mod browse_description;
mod browse_request;
mod browse_response;
mod browse_result;
mod create_monitored_items_request;
mod create_monitored_items_response;
mod create_subscription_request;
mod create_subscription_respones;
mod data_value;
mod date_time;
mod delete_monitored_items_request;
mod delete_monitored_items_response;
mod delete_subscriptions_request;
mod delete_subscriptions_response;
mod expanded_node_id;
mod localized_text;
mod monitored_item_create_request;
mod monitored_item_create_result;
mod node_id;
mod qualified_name;
mod read_request;
mod read_response;
mod read_value_id;
mod reference_description;
mod string;
mod variant;
mod write_request;
mod write_response;
mod write_value;

pub use self::{
    browse_description::BrowseDescription, browse_request::BrowseRequest,
    browse_response::BrowseResponse, browse_result::BrowseResult,
    create_monitored_items_request::CreateMonitoredItemsRequest,
    create_monitored_items_response::CreateMonitoredItemsResponse,
    create_subscription_request::CreateSubscriptionRequest,
    create_subscription_respones::CreateSubscriptionResponse, data_value::DataValue,
    date_time::DateTime, delete_monitored_items_request::DeleteMonitoredItemsRequest,
    delete_monitored_items_response::DeleteMonitoredItemsResponse,
    delete_subscriptions_request::DeleteSubscriptionsRequest,
    delete_subscriptions_response::DeleteSubscriptionsResponse, expanded_node_id::ExpandedNodeId,
    localized_text::LocalizedText, monitored_item_create_request::MonitoredItemCreateRequest,
    monitored_item_create_result::MonitoredItemCreateResult, node_id::NodeId,
    qualified_name::QualifiedName, read_request::ReadRequest, read_response::ReadResponse,
    read_value_id::ReadValueId, reference_description::ReferenceDescription, string::String,
    variant::Variant, write_request::WriteRequest, write_response::WriteResponse,
    write_value::WriteValue,
};

macro_rules! primitive {
    ($name:ident, $type:ty) => {
        paste::paste! {
            crate::data_type!($name, [<UA_ $name>], [<UA_TYPES_ $name:upper>]);
        }

        impl $name {
            #[must_use]
            pub fn new(value: $type) -> Self {
                Self::from_ref(&value)
            }
        }
    };
}

primitive!(Boolean, bool);
primitive!(SByte, i8);
primitive!(Byte, u8);
primitive!(Int16, i16);
primitive!(UInt16, u16);
primitive!(Int32, i32);
primitive!(UInt32, u32);
primitive!(Int64, i64);
primitive!(UInt64, u64);
primitive!(Float, f32);
primitive!(Double, f64);
