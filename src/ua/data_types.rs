//! Thin wrappers for OPC UA data types from [`open62541_sys`].

mod attribute_id;
mod browse_description;
mod browse_direction;
mod browse_request;
mod browse_response;
mod browse_result;
mod browse_result_mask;
mod call_method_request;
mod call_method_result;
mod call_request;
mod call_response;
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
mod node_class;
mod node_id;
mod node_id_type;
mod qualified_name;
mod read_request;
mod read_response;
mod read_value_id;
mod reference_description;
mod status_code;
mod string;
mod timestamps_to_return;
mod variant;
mod write_request;
mod write_response;
mod write_value;

pub use self::{
    attribute_id::AttributeId, browse_description::BrowseDescription,
    browse_direction::BrowseDirection, browse_request::BrowseRequest,
    browse_response::BrowseResponse, browse_result::BrowseResult,
    browse_result_mask::BrowseResultMask, call_method_request::CallMethodRequest,
    call_method_result::CallMethodResult, call_request::CallRequest, call_response::CallResponse,
    create_monitored_items_request::CreateMonitoredItemsRequest,
    create_monitored_items_response::CreateMonitoredItemsResponse,
    create_subscription_request::CreateSubscriptionRequest,
    create_subscription_respones::CreateSubscriptionResponse, data_value::DataValue,
    date_time::DateTime, delete_monitored_items_request::DeleteMonitoredItemsRequest,
    delete_monitored_items_response::DeleteMonitoredItemsResponse,
    delete_subscriptions_request::DeleteSubscriptionsRequest,
    delete_subscriptions_response::DeleteSubscriptionsResponse, expanded_node_id::ExpandedNodeId,
    localized_text::LocalizedText, monitored_item_create_request::MonitoredItemCreateRequest,
    monitored_item_create_result::MonitoredItemCreateResult, node_class::NodeClass,
    node_id::NodeId, node_id_type::NodeIdType, qualified_name::QualifiedName,
    read_request::ReadRequest, read_response::ReadResponse, read_value_id::ReadValueId,
    reference_description::ReferenceDescription, status_code::StatusCode, string::String,
    timestamps_to_return::TimestampsToReturn, variant::Variant, write_request::WriteRequest,
    write_response::WriteResponse, write_value::WriteValue,
};

macro_rules! primitive {
    ($( ($name:ident, $type:ty) ),* $(,)?) => {
        $(
            paste::paste! {
                crate::data_type!($name);
            }

            impl $name {
                #[must_use]
                pub const fn new(value: $type) -> Self {
                    $name(value)
                }

                #[must_use]
                pub const fn value(&self) -> $type {
                    self.0
                }
            }
        )*
    };
}

primitive!(
    (Boolean, bool), // Data type ns=0;i=1
    (SByte, i8),     // Data type ns=0;i=2
    (Byte, u8),      // Data type ns=0;i=3
    (Int16, i16),    // Data type ns=0;i=4
    (UInt16, u16),   // Data type ns=0;i=5
    (Int32, i32),    // Data type ns=0;i=6
    (UInt32, u32),   // Data type ns=0;i=7
    (Int64, i64),    // Data type ns=0;i=8
    (UInt64, u64),   // Data type ns=0;i=9
    (Float, f32),    // Data type ns=0;i=10
    (Double, f64),   // Data type ns=0;i=11
);
