use std::fmt::Debug;

use open62541_sys::UA_DataType;

use crate::{ua, DataType};

/// Extended values.
///
/// This is used for values which are represented differently from their underlying data type, e.g.
/// [`AttributeWriteMask`] which is [`UInt32`] with additional methods and particular semantics.
///
/// [`AttributeWriteMask`]: crate::ua::AttributeWriteMask
/// [`UInt32`]: crate::ua::UInt32
pub trait DataTypeExt: Debug + Clone {
    /// Inner type sent over the wire.
    type Inner: DataType;

    /// Creates instance for immer type.
    fn from_inner(value: Self::Inner) -> Self;

    /// Returns inner type representation.
    fn into_inner(self) -> Self::Inner;
}

// Umbrella implementation that simplifies type constraints: `DataType` is trivially `DataTypeExt`.
impl<T: DataType> DataTypeExt for T {
    type Inner = Self;

    fn from_inner(value: Self::Inner) -> Self {
        value
    }

    fn into_inner(self) -> Self::Inner {
        self
    }
}

/// Node attribute.
///
/// This is used to match the appropriate result types at compile time when reading attributes from
/// nodes. See the following methods for details:
///
/// - [`AsyncClient::read_attribute()`](crate::AsyncClient::read_attribute)
/// - [`Server::read_attribute()`](crate::Server::read_attribute)
//
// FIXME: Turn into sealed trait.
pub trait Attribute: Debug + Copy {
    /// Attribute data type.
    type Value: DataTypeExt;

    /// Gets attribute ID.
    fn id(&self) -> ua::AttributeId;
}

/// Server node attributes.
///
/// This is used to allow handling different node types when adding nodes to the server's data tree
/// in [`Server::add_node()`](crate::Server::add_node).
//
// FIXME: Turn into sealed trait.
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
pub trait PrivateKeyPasswordCallback {
    /// Provides private-key password.
    ///
    /// # Errors
    ///
    /// This should return an appropriate error when the password cannot be provided.
    fn private_key_password(&self) -> Result<crate::Password, crate::Error>;
}

#[cfg(feature = "mbedtls")]
impl<F> PrivateKeyPasswordCallback for F
where
    F: Fn() -> Result<crate::Password, crate::Error>,
{
    fn private_key_password(&self) -> Result<crate::Password, crate::Error> {
        self()
    }
}

/// Monitoring filter.
///
/// This is used as extensible parameter in [`ua::MonitoringParameters::with_filter()`].
pub trait MonitoringFilter: Debug + Send + Sync + 'static {
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
pub trait FilterOperand: Debug + Send + Sync + 'static {
    fn to_extension_object(&self) -> ua::ExtensionObject;
}

impl FilterOperand for Box<dyn FilterOperand> {
    fn to_extension_object(&self) -> ua::ExtensionObject {
        (**self).to_extension_object()
    }
}
