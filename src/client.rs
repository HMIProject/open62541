use std::{ffi::CString, ptr, time::Duration};

use open62541_sys::{
    UA_CertificateGroup_AcceptAll, UA_Client_connect, UA_Client_getConfig, UA_Client_getEndpoints,
    UA_Client_getRemoteDataTypes, UA_ClientConfig,
};

use crate::{DataType as _, Error, Result, ua};

/// Builder for [`Client`].
///
/// Use this to specify additional options when connecting to an OPC UA endpoint.
///
/// # Examples
///
/// ```no_run
/// use open62541::ClientBuilder;
/// use std::time::Duration;
///
/// # #[tokio::main]
/// # async fn main() -> anyhow::Result<()> {
/// #
/// let client = ClientBuilder::default()
///     .secure_channel_life_time(Duration::from_secs(60))
///     .connect("opc.tcp://opcuademo.sterfive.com:26543")?;
/// #
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct ClientBuilder(ua::ClientConfig);

impl ClientBuilder {
    /// Creates builder from default client config.
    // Method name refers to call of `UA_ClientConfig_setDefault()`.
    #[must_use]
    fn default() -> Self {
        Self(ua::ClientConfig::default(ClientContext::default()))
    }

    /// Creates builder from default client config with encryption.
    ///
    /// This requires certificate and associated private key data in [DER] or [PEM] format. Data may
    /// be read from local files or created with [`crate::create_certificate()`].
    ///
    /// ```
    /// use open62541::{Certificate, ClientBuilder, PrivateKey};
    ///
    /// const CERTIFICATE_PEM: &[u8] = include_bytes!("../examples/client_certificate.pem");
    /// const PRIVATE_KEY_PEM: &[u8] = include_bytes!("../examples/client_private_key.pem");
    ///
    /// let certificate = Certificate::from_bytes(CERTIFICATE_PEM);
    /// let private_key = PrivateKey::from_bytes(PRIVATE_KEY_PEM);
    ///
    /// # let _ = move || -> open62541::Result<()> {
    /// let client = ClientBuilder::default_encryption(&certificate, &private_key)
    ///     .expect("should create builder with encryption")
    ///     .connect("opc.tcp://localhost")?;
    /// # Ok(())
    /// # };
    /// ```
    ///
    /// # Errors
    ///
    /// This fails when the certificate is invalid or the private key cannot be decrypted (e.g. when
    /// it has been protected by a password).
    ///
    /// [DER]: https://en.wikipedia.org/wiki/X.690#DER_encoding
    /// [PEM]: https://en.wikipedia.org/wiki/Privacy-Enhanced_Mail
    // Method name refers to call of `UA_ClientConfig_setDefaultEncryption()`.
    #[cfg(feature = "mbedtls")]
    pub fn default_encryption(
        local_certificate: &crate::Certificate,
        private_key: &crate::PrivateKey,
    ) -> Result<Self> {
        Ok(Self(ua::ClientConfig::default_encryption(
            ClientContext::default(),
            local_certificate,
            private_key,
        )?))
    }

    /// Sets (response) timeout.
    ///
    /// # Panics
    ///
    /// The given duration must be non-negative and less than 4,294,967,295 milliseconds (less than
    /// 49.7 days).
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config_mut().timeout = u32::try_from(timeout.as_millis())
            .expect("timeout (in milliseconds) should be in range of u32");
        self
    }

    /// Sets client description.
    ///
    /// The description must be internally consistent. The application URI set in the application
    /// description must match the URI set in the certificate.
    #[must_use]
    pub fn client_description(mut self, client_description: ua::ApplicationDescription) -> Self {
        client_description.move_into_raw(&mut self.config_mut().clientDescription);
        self
    }

    /// Sets user identity token.
    #[must_use]
    pub fn user_identity_token(mut self, user_identity_token: &ua::UserIdentityToken) -> Self {
        user_identity_token
            .to_extension_object()
            .move_into_raw(&mut self.config_mut().userIdentityToken);
        self
    }

    /// Sets security mode.
    #[must_use]
    pub fn security_mode(mut self, security_mode: ua::MessageSecurityMode) -> Self {
        security_mode.move_into_raw(&mut self.config_mut().securityMode);
        self
    }

    /// Sets security policy URI.
    ///
    /// The known values are as follows:
    ///
    /// - `http://opcfoundation.org/UA/SecurityPolicy#None`
    /// - `http://opcfoundation.org/UA/SecurityPolicy#Basic128Rsa15` (deprecated)
    /// - `http://opcfoundation.org/UA/SecurityPolicy#Basic256` (deprecated)
    /// - `http://opcfoundation.org/UA/SecurityPolicy#Basic256Sha256`
    /// - `http://opcfoundation.org/UA/SecurityPolicy#Aes128_Sha256_RsaOaep`
    /// - `http://opcfoundation.org/UA/SecurityPolicy#Aes256_Sha256_RsaPss`
    #[must_use]
    pub fn security_policy_uri(mut self, security_policy_uri: ua::String) -> Self {
        security_policy_uri.move_into_raw(&mut self.config_mut().securityPolicyUri);
        self
    }

    /// Sets secure channel life time.
    ///
    /// After this life time, the channel needs to be renewed.
    ///
    /// # Panics
    ///
    /// The given duration must be non-negative and less than 4,294,967,295 milliseconds (less than
    /// 49.7 days).
    #[must_use]
    pub fn secure_channel_life_time(mut self, secure_channel_life_time: Duration) -> Self {
        self.config_mut().secureChannelLifeTime =
            u32::try_from(secure_channel_life_time.as_millis())
                .expect("secure channel life time (in milliseconds) should be in range of u32");
        self
    }

    /// Sets requested session timeout.
    ///
    /// # Panics
    ///
    /// The given duration must be non-negative and less than 4,294,967,295 milliseconds (less than
    /// 49.7 days).
    #[must_use]
    pub fn requested_session_timeout(mut self, requested_session_timeout: Duration) -> Self {
        self.config_mut().requestedSessionTimeout =
            u32::try_from(requested_session_timeout.as_millis())
                .expect("secure channel life time (in milliseconds) should be in range of u32");
        self
    }

    /// Sets connectivity check interval.
    ///
    /// Use `None` to disable background task.
    ///
    /// # Panics
    ///
    /// The given duration must be non-negative and less than 4,294,967,295 milliseconds (less than
    /// 49.7 days).
    #[must_use]
    pub fn connectivity_check_interval(
        mut self,
        connectivity_check_interval: Option<Duration>,
    ) -> Self {
        self.config_mut().connectivityCheckInterval =
            u32::try_from(connectivity_check_interval.map_or(0, |interval| interval.as_millis()))
                .expect("connectivity check interval (in milliseconds) should be in range of u32");
        self
    }

    /// Disables server certificate checks.
    ///
    /// Note that this disables all certificate verification of server communications. Use only when
    /// servers can be identified in some other way, or identity is not relevant.
    ///
    /// This is a shortcut for using [`certificate_verification()`](Self::certificate_verification)
    /// with [`ua::CertificateVerification::accept_all()`].
    #[must_use]
    pub fn accept_all(mut self) -> Self {
        let config = self.config_mut();
        unsafe {
            UA_CertificateGroup_AcceptAll(&raw mut config.certificateVerification);
        }
        self
    }

    /// Sets certificate verification.
    #[must_use]
    pub fn certificate_verification(
        mut self,
        certificate_verification: ua::CertificateVerification,
    ) -> Self {
        let config = self.config_mut();
        certificate_verification.move_into_raw(&mut config.certificateVerification);
        self
    }

    /// Sets private-key password callback.
    ///
    /// The trait [`PrivateKeyPasswordCallback`] is implemented for closures, so this call may be as
    /// simple as the following (note that hard-coding secrets like this is not secure and should be
    /// avoided):
    ///
    /// ```
    /// # use open62541::{Certificate, ClientBuilder, PrivateKey};
    /// #
    /// # let _ = |certificate: Certificate, private_key: PrivateKey| -> open62541::Result<()> {
    /// let client = ClientBuilder::default_encryption(&certificate, &private_key)
    ///     .expect("should create builder with encryption")
    ///     .private_key_password_callback(|| Ok(String::from("secret").into()))
    ///     .connect("opc.tcp://localhost")?;
    /// # Ok(())
    /// # };
    /// ```
    ///
    /// [`PrivateKeyPasswordCallback`]: crate::PrivateKeyPasswordCallback
    #[cfg(feature = "mbedtls")]
    #[must_use]
    pub fn private_key_password_callback(
        mut self,
        private_key_password_callback: impl crate::PrivateKeyPasswordCallback + 'static,
    ) -> Self {
        self.context_mut().private_key_password_callback =
            Some(Box::new(private_key_password_callback));
        self
    }

    /// Connects to OPC UA endpoint and returns [`Client`].
    ///
    /// # Errors
    ///
    /// This fails when the target server is not reachable.
    ///
    /// # Panics
    ///
    /// The endpoint URL must not contain any NUL bytes.
    pub fn connect(self, endpoint_url: &str) -> Result<Client> {
        let mut client = self.build();
        client.connect(endpoint_url)?;
        Ok(client)
    }

    /// Connects to OPC UA server and returns endpoints.
    ///
    /// # Errors
    ///
    /// This fails when the target server is not reachable.
    ///
    /// # Panics
    ///
    /// The server URL must not contain any NUL bytes.
    pub fn get_endpoints(self, server_url: &str) -> Result<ua::Array<ua::EndpointDescription>> {
        log::info!("Getting endpoints of server {server_url}");

        let server_url = CString::new(server_url).expect("server URL does not contain NUL bytes");

        let mut client = self.build();
        let endpoint_descriptions: Option<ua::Array<ua::EndpointDescription>>;

        let status_code = ua::StatusCode::new({
            let mut endpoint_descriptions_size = 0;
            let mut endpoint_descriptions_ptr = ptr::null_mut();
            let result = unsafe {
                UA_Client_getEndpoints(
                    client.0.as_mut_ptr(),
                    server_url.as_ptr(),
                    &raw mut endpoint_descriptions_size,
                    &raw mut endpoint_descriptions_ptr,
                )
            };
            // Wrap array result immediately to not leak memory when leaving function early as with
            // `?` below.
            endpoint_descriptions = ua::Array::<ua::EndpointDescription>::from_raw_parts(
                endpoint_descriptions_size,
                endpoint_descriptions_ptr,
            );
            result
        });
        Error::verify_good(&status_code)?;

        let Some(endpoint_descriptions) = endpoint_descriptions else {
            return Err(Error::internal("expected array of endpoint descriptions"));
        };

        Ok(endpoint_descriptions)
    }

    /// Builds OPC UA client.
    #[must_use]
    fn build(self) -> Client {
        Client(ua::Client::new_with_config(self.0))
    }

    /// Access client configuration.
    #[must_use]
    fn config_mut(&mut self) -> &mut UA_ClientConfig {
        // SAFETY: Ownership is not given away.
        unsafe { self.0.as_mut() }
    }

    /// Access client context.
    #[cfg_attr(not(feature = "mbedtls"), expect(dead_code, reason = "unused"))]
    #[must_use]
    fn context_mut(&mut self) -> &mut ClientContext {
        self.0.context_mut()
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::default()
    }
}

/// Custom client context.
///
/// This is stored in the client's `clientContext` attribute and manages custom callbacks as used by
/// [`ClientBuilder::private_key_password_callback()`].
#[derive(Default)]
pub(crate) struct ClientContext {
    #[cfg(feature = "mbedtls")]
    pub(crate) private_key_password_callback: Option<Box<dyn crate::PrivateKeyPasswordCallback>>,
}

/// Connected OPC UA client.
///
/// This represents an OPC UA client connected to a specific endpoint. Once a client is connected to
/// an endpoint, it is not possible to switch to another server. Create a new client for that.
///
/// Once a connection to the given endpoint is established, the client keeps the connection open and
/// reconnects if necessary.
///
/// If the connection fails unrecoverably, the client is no longer usable. In this case create a new
/// client if required.
///
/// To disconnect, prefer method [`disconnect()`](Self::disconnect) over simply dropping the client:
/// disconnection involves server communication and might take a short amount of time.
#[derive(Debug)]
pub struct Client(ua::Client);

impl Client {
    /// Creates default client connected to endpoint.
    ///
    /// If you need more control over the initialization, use [`ClientBuilder`] instead, and turn it
    /// into [`Client`] by calling [`connect()`](ClientBuilder::connect).
    ///
    /// # Errors
    ///
    /// See [`ClientBuilder::connect()`].
    ///
    /// # Panics
    ///
    /// See [`ClientBuilder::connect()`].
    pub fn new(endpoint_url: &str) -> Result<Self> {
        ClientBuilder::default().connect(endpoint_url)
    }

    /// Turns client into [`AsyncClient`].
    ///
    /// The [`AsyncClient`] can be used to access methods in an asynchronous way.
    ///
    /// [`AsyncClient`]: crate::AsyncClient
    #[must_use]
    pub fn into_async(self) -> crate::AsyncClient {
        crate::AsyncClient::from_sync(self.0)
    }

    /// Gets current channel and session state, and connect status.
    #[must_use]
    pub fn state(&self) -> ua::ClientState {
        self.0.state()
    }

    pub fn with_remote_data_types(mut self) -> Result<Self> {
        let mut out_custom_data_types = ptr::null_mut();

        let status_code = ua::StatusCode::new(unsafe {
            UA_Client_getRemoteDataTypes(
                self.0.as_mut_ptr(),
                0,
                ptr::null(),
                &raw mut out_custom_data_types,
            )
        });
        Error::verify_good(&status_code)?;

        let config = unsafe { UA_Client_getConfig(self.0.as_mut_ptr()) };
        let config = unsafe { config.as_mut() }.expect("client config must be set");

        assert!(
            config.customDataTypes.is_null(),
            "custom data types already set"
        );
        config.customDataTypes = out_custom_data_types;

        Ok(self)
    }

    /// Connects to endpoint.
    ///
    /// This method is always called internally before passing new [`Client`] instances to the user:
    /// our contract states that a `Client` should always be connected.
    fn connect(&mut self, endpoint_url: &str) -> Result<()> {
        log::info!("Connecting to endpoint {endpoint_url}");

        let endpoint_url =
            CString::new(endpoint_url).expect("endpoint URL does not contain NUL bytes");

        let status_code = ua::StatusCode::new(unsafe {
            // SAFETY: The method does not take ownership of `client`.
            UA_Client_connect(self.0.as_mut_ptr(), endpoint_url.as_ptr())
        });
        Error::verify_good(&status_code)
    }

    /// Disconnects from endpoint.
    ///
    /// This consumes the client and handles the graceful shutdown of the connection. This should be
    /// preferred over simply dropping the instance to give the server a chance to clean up and also
    /// to avoid blocking unexpectedly when the client is being dropped without calling this method.
    #[expect(clippy::semicolon_if_nothing_returned, reason = "future fail-safe")]
    // Forward any result as-is to detect mismatching method signatures at compile time if the
    // return type of the inner method should ever change.
    pub fn disconnect(self) {
        self.0.disconnect()
    }
}
