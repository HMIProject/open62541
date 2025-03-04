use std::fmt;

use open62541_sys::UA_DataType;

use crate::{ua, DataType};

/// Node attribute.
///
/// This is used to match the appropriate result types at compile time when reading attributes from
/// nodes. See the following methods for details:
///
/// - [`AsyncClient::read_attribute()`](crate::AsyncClient::read_attribute)
/// - [`Server::read_attribute()`](crate::Server::read_attribute)
pub trait Attribute: fmt::Debug + Copy {
    /// Attribute data type.
    type Value: DataType;

    /// Gets attribute ID.
    fn id(&self) -> ua::AttributeId;
}

/// Server node attributes.
///
/// This is used to allow handling different node types when adding nodes to the server's data tree
/// in [`Server::add_node()`](crate::Server::add_node).
pub trait Attributes: DataType {
    /// Gets associated node class.
    fn node_class(&self) -> ua::NodeClass;

    /// Gets associated attribute type.
    ///
    /// This is [`<Self as DataType>::data_type()`] with a more appropriate name.
    ///
    /// [`<Self as DataType>::data_type()`]: DataType::data_type()
    fn attribute_type(&self) -> *const UA_DataType;

    /// Sets display name.
    #[must_use]
    fn with_display_name(self, display_name: &ua::LocalizedText) -> Self;

    /// Gets generic [`ua::NodeAttributes`] type.
    fn as_node_attributes(&self) -> &ua::NodeAttributes;
}

/// Custom certificate verification.
///
/// This is used to implement custom callbacks in [`ua::CertificateVerification::custom()`].
pub trait CustomCertificateVerification {
    fn verify_certificate(&self, certificate: &ua::ByteString) -> ua::StatusCode;

    fn verify_application_uri(
        &self,
        certificate: &ua::ByteString,
        application_uri: &ua::String,
    ) -> ua::StatusCode;
}

/// Private-key password callback.
///
/// This is used to fetch the password for a given client private key when establishing a connection
/// in [`Client`] or [`AsyncClient`].
///
/// See [`ClientBuilder::private_key_password_callback()`] for details.
///
/// [`Client`]: crate::Client
/// [`AsyncClient`]: crate::AsyncClient
/// [`ClientBuilder::private_key_password_callback()`]: crate::ClientBuilder::private_key_password_callback
#[cfg(feature = "mbedtls")]
pub trait PrivateKeyPasswordCallback: fmt::Debug {
    /// Provides private key password.
    ///
    /// # Errors
    ///
    /// This should return an appropriate error when the password cannot be provided.
    fn private_key_password(&self) -> Result<crate::Password, crate::Error>;
}

/// Monitoring filter.
///
/// This is used as extensible parameter in [`ua::MonitoringParameters::with_filter()`].
pub trait MonitoringFilter: fmt::Debug + Send + Sync + 'static {
    fn to_extension_object(&self) -> ua::ExtensionObject;
}

impl MonitoringFilter for Box<dyn MonitoringFilter> {
    fn to_extension_object(&self) -> ua::ExtensionObject {
        (**self).to_extension_object()
    }
}

/// Filter operand.
///
/// This is used as extensible parameter in [`ua::ContentFilterElement::with_filter_operands()`].
pub trait FilterOperand: fmt::Debug + Send + Sync + 'static {
    fn to_extension_object(&self) -> ua::ExtensionObject;
}

impl FilterOperand for Box<dyn FilterOperand> {
    fn to_extension_object(&self) -> ua::ExtensionObject {
        (**self).to_extension_object()
    }
}
