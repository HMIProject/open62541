//! Thin wrappers for OPC UA data types from [`open62541_sys`].

mod aggregate_filter;
mod anonymous_identity_token;
mod application_description;
mod application_type;
mod argument;
mod attribute_id;
mod attribute_operand;
mod browse_description;
mod browse_direction;
mod browse_next_request;
mod browse_next_response;
mod browse_path;
mod browse_path_result;
mod browse_path_target;
mod browse_request;
mod browse_response;
mod browse_result;
mod build_info;
mod byte_string;
mod call_method_request;
mod call_method_result;
mod call_request;
mod call_response;
mod content_filter;
mod content_filter_element;
mod create_monitored_items_request;
mod create_monitored_items_response;
mod create_subscription_request;
mod create_subscription_respones;
mod data_change_filter;
mod data_value;
mod date_time;
mod delete_monitored_items_request;
mod delete_monitored_items_response;
mod delete_subscriptions_request;
mod delete_subscriptions_response;
mod element_operand;
mod endpoint_description;
mod enum_definition;
mod enum_description;
mod enum_field;
mod enumeration;
mod eu_information;
mod event_filter;
mod expanded_node_id;
mod extension_object;
mod filter_operator;
mod guid;
mod issued_identity_token;
mod literal_operand;
mod localized_text;
mod message_security_mode;
mod monitored_item_create_request;
mod monitored_item_create_result;
mod monitoring_mode;
mod monitoring_parameters;
mod node_attributes;
mod node_class;
mod node_id;
mod node_id_type;
mod qualified_name;
mod range;
mod read_request;
mod read_response;
mod read_value_id;
mod reference_description;
mod relative_path;
mod relative_path_element;
mod request_header;
mod response_header;
mod server_state;
mod server_status_data_type;
mod simple_attribute_operand;
mod status_code;
mod string;
mod structure_definition;
mod structure_description;
mod structure_field;
mod timestamps_to_return;
mod trust_list_data_type;
mod user_name_identity_token;
mod variant;
mod write_request;
mod write_response;
mod write_value;
mod x509_identity_token;

pub use self::{
    aggregate_filter::AggregateFilter,
    anonymous_identity_token::AnonymousIdentityToken,
    application_description::ApplicationDescription,
    application_type::ApplicationType,
    argument::Argument,
    attribute_id::AttributeId,
    attribute_operand::AttributeOperand,
    browse_description::BrowseDescription,
    browse_direction::BrowseDirection,
    browse_next_request::BrowseNextRequest,
    browse_next_response::BrowseNextResponse,
    browse_path::BrowsePath,
    browse_path_result::BrowsePathResult,
    browse_path_target::BrowsePathTarget,
    browse_request::BrowseRequest,
    browse_response::BrowseResponse,
    browse_result::BrowseResult,
    build_info::BuildInfo,
    byte_string::ByteString,
    call_method_request::CallMethodRequest,
    call_method_result::CallMethodResult,
    call_request::CallRequest,
    call_response::CallResponse,
    content_filter::ContentFilter,
    content_filter_element::ContentFilterElement,
    create_monitored_items_request::CreateMonitoredItemsRequest,
    create_monitored_items_response::CreateMonitoredItemsResponse,
    create_subscription_request::CreateSubscriptionRequest,
    create_subscription_respones::CreateSubscriptionResponse,
    data_change_filter::DataChangeFilter,
    data_value::DataValue,
    date_time::DateTime,
    delete_monitored_items_request::DeleteMonitoredItemsRequest,
    delete_monitored_items_response::DeleteMonitoredItemsResponse,
    delete_subscriptions_request::DeleteSubscriptionsRequest,
    delete_subscriptions_response::DeleteSubscriptionsResponse,
    element_operand::ElementOperand,
    endpoint_description::EndpointDescription,
    enum_definition::EnumDefinition,
    enum_description::EnumDescription,
    enum_field::EnumField,
    enumeration::Enumeration,
    eu_information::EUInformation,
    event_filter::EventFilter,
    expanded_node_id::ExpandedNodeId,
    extension_object::ExtensionObject,
    filter_operator::FilterOperator,
    guid::Guid,
    issued_identity_token::IssuedIdentityToken,
    literal_operand::LiteralOperand,
    localized_text::LocalizedText,
    message_security_mode::MessageSecurityMode,
    monitored_item_create_request::MonitoredItemCreateRequest,
    monitored_item_create_result::MonitoredItemCreateResult,
    monitoring_mode::MonitoringMode,
    monitoring_parameters::MonitoringParameters,
    node_attributes::{
        DataTypeAttributes, MethodAttributes, NodeAttributes, ObjectAttributes,
        ObjectTypeAttributes, ReferenceTypeAttributes, VariableAttributes, VariableTypeAttributes,
        ViewAttributes,
    },
    node_class::NodeClass,
    node_id::NodeId,
    node_id_type::NodeIdType,
    qualified_name::QualifiedName,
    range::Range,
    read_request::ReadRequest,
    read_response::ReadResponse,
    read_value_id::ReadValueId,
    reference_description::ReferenceDescription,
    relative_path::RelativePath,
    relative_path_element::RelativePathElement,
    request_header::RequestHeader,
    response_header::ResponseHeader,
    server_state::ServerState,
    server_status_data_type::ServerStatusDataType,
    simple_attribute_operand::SimpleAttributeOperand,
    status_code::StatusCode,
    string::String,
    structure_definition::StructureDefinition,
    structure_description::StructureDescription,
    structure_field::StructureField,
    timestamps_to_return::TimestampsToReturn,
    trust_list_data_type::TrustListDataType,
    user_name_identity_token::UserNameIdentityToken,
    variant::Variant,
    write_request::WriteRequest,
    write_response::WriteResponse,
    write_value::WriteValue,
    x509_identity_token::X509IdentityToken,
};

macro_rules! primitive {
    ($( ($name:ident, $type:ty) ),* $(,)?) => {
        $(
            $crate::data_type!($name);

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

            #[cfg(feature = "serde")]
            impl serde::Serialize for $name {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    paste::paste! {
                        serializer.[<serialize_ $type>](self.0)
                    }
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
