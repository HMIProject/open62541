use std::{ffi::CString, time::Duration};

use open62541_sys::{UA_ClientConfig, UA_Client_connect};

use crate::{ua, DataType as _, Error, Result};

/// Builder for [`Client`].
///
/// Use this to specify additional options before connecting to an OPC UA endpoint.
///
/// # Examples
///
/// ```no_run
/// use open62541::ClientBuilder;
/// use std::time::Duration;
///
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() -> anyhow::Result<()> {
/// #
/// let client = ClientBuilder::default()
///     .secure_channel_life_time(Duration::from_secs(60))
///     .connect("opc.tcp://opcuademo.sterfive.com:26543")?;
/// #
/// # Ok(())
/// # }
/// ```
#[derive(Default)]
#[allow(clippy::module_name_repetitions)]
pub struct ClientBuilder(ua::ClientConfig);

impl ClientBuilder {
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
        log::info!("Connecting to endpoint {endpoint_url}");

        let endpoint_url =
            CString::new(endpoint_url).expect("endpoint URL does not contain NUL bytes");

        let mut client = ua::Client::new_with_config(self.0);

        let status_code = ua::StatusCode::new(unsafe {
            UA_Client_connect(client.as_mut_ptr(), endpoint_url.as_ptr())
        });
        Error::verify_good(&status_code)?;

        Ok(Client(client))
    }

    /// Access client configuration.
    fn config_mut(&mut self) -> &mut UA_ClientConfig {
        // SAFETY: Ownership is not given away.
        unsafe { self.0.as_mut() }
    }
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
pub struct Client(
    #[allow(dead_code)] // --no-default-features
    ua::Client,
);

impl Client {
    /// Creates client connected to endpoint.
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
    /// `cycle_time` controls the frequency at which the client will poll the server for responses
    /// in the background.
    ///
    /// [`AsyncClient`]: crate::AsyncClient
    #[cfg(feature = "tokio")]
    #[must_use]
    pub fn into_async(self, cycle_time: tokio::time::Duration) -> crate::AsyncClient {
        crate::AsyncClient::from_sync(self.0, cycle_time)
    }

    /// Disconnects from endpoint.
    ///
    /// This consumes the client and handles the graceful shutdown of the connection. This should be
    /// preferred over simply dropping the instance to give the server a chance to clean up and also
    /// to avoid blocking unexpectedly when the client is being dropped without calling this method.
    // Forward any result as-is to detect mismatching method signatures at compile time if the
    // return type of the inner method should ever change.
    #[allow(clippy::semicolon_if_nothing_returned)]
    pub fn disconnect(self) {
        self.0.disconnect()
    }
}
