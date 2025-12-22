//! Thin wrappers for [`open62541_sys`] types.

mod access_level_ex_type;
mod access_level_type;
mod access_restriction_type;
mod array;
mod attribute_write_mask;
mod browse_result_mask;
#[cfg(feature = "mbedtls")]
mod certificate_format;
mod certificate_verification;
mod client;
mod client_config;
mod continuation_point;
mod data_type_definition;
mod data_types;
mod duration;
mod event_id;
mod event_notifier_type;
mod integer_id;
mod key_value_map;
mod logger;
mod monitored_item_id;
mod node_class_mask;
mod secure_channel_state;
mod secure_channel_statistics;
mod security_level;
mod server;
mod server_config;
mod server_statistics;
mod session_state;
mod session_statistics;
mod specified_attributes;
mod subscription_id;
mod trust_list_masks;
mod unit_id;
mod user_identity_token;

#[cfg(feature = "mbedtls")]
pub use self::certificate_format::CertificateFormat;
#[expect(deprecated, reason = "explicit re-export")]
pub use self::{
    access_level_ex_type::AccessLevelExType,
    access_level_type::{AccessLevel, AccessLevelType},
    access_restriction_type::AccessRestrictionType,
    array::Array,
    attribute_write_mask::AttributeWriteMask,
    browse_result_mask::BrowseResultMask,
    certificate_verification::CertificateVerification,
    client::{Client, ClientState},
    continuation_point::ContinuationPoint,
    data_type_definition::DataTypeDefinition,
    data_types::*,
    duration::Duration,
    event_id::EventId,
    event_notifier_type::EventNotifierType,
    integer_id::IntegerId,
    key_value_map::KeyValueMap,
    monitored_item_id::MonitoredItemId,
    node_class_mask::NodeClassMask,
    secure_channel_state::SecureChannelState,
    secure_channel_statistics::SecureChannelStatistics,
    security_level::SecurityLevel,
    server::Server,
    server_statistics::ServerStatistics,
    session_state::SessionState,
    session_statistics::SessionStatistics,
    specified_attributes::SpecifiedAttributes,
    subscription_id::SubscriptionId,
    trust_list_masks::TrustListMasks,
    unit_id::UnitId,
    user_identity_token::UserIdentityToken,
};
pub(crate) use self::{client_config::ClientConfig, logger::Logger, server_config::ServerConfig};
