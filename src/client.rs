use std::ffi::CString;

use log::info;
use open62541_sys::{
    UA_AttributeId_UA_ATTRIBUTEID_NODEID, UA_AttributeId_UA_ATTRIBUTEID_VALUE,
    UA_ClientConfig_setDefault, UA_Client_Service_read, UA_Client_connect, UA_Client_getConfig,
    __UA_Client_readAttribute, UA_STATUSCODE_GOOD, UA_TYPES, UA_TYPES_NODEID, UA_TYPES_VARIANT,
};

use crate::{ua, Error};

/// Builder for [`Client`].
///
/// Use this to specify additional options before connecting to an OPC UA endpoint.
#[allow(clippy::module_name_repetitions)]
pub struct ClientBuilder(ua::Client);

impl ClientBuilder {
    /// Connects to OPC UA endpoint and returns [`Client`].
    ///
    /// # Errors
    ///
    /// This fails when the target server is not reachable.
    ///
    /// # Panics
    ///
    /// The endpoint URL must be a valid C string, i.e. it must not contain any NUL bytes.
    pub fn connect(mut self, endpoint_url: &str) -> Result<Client, Error> {
        info!("Connecting to endpoint {endpoint_url}");

        let endpoint_url =
            CString::new(endpoint_url).expect("endpoint URL does not contain NUL bytes");

        let result = unsafe { UA_Client_connect(self.0.as_mut_ptr(), endpoint_url.as_ptr()) };
        if result != UA_STATUSCODE_GOOD {
            return Err(Error::new(result));
        }

        Ok(Client(self.0))
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        let mut inner = ua::Client::default();

        // Clients need to be initialized with config for `UA_Client_connect` to work.
        let result = unsafe {
            let config = UA_Client_getConfig(inner.as_mut_ptr());
            UA_ClientConfig_setDefault(config)
        };
        assert!(result == UA_STATUSCODE_GOOD);

        Self(inner)
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
pub struct Client(ua::Client);

impl Client {
    /// Creates client connected to endpoint.
    ///
    /// If you need more control over the initialization, use [`ClientBuilder`] instead, and turn it
    /// into [`Client`] by calling [`connect()`](ClientBuilder::connect()).
    ///
    /// # Errors
    ///
    /// See [`ClientBuilder::connect()`].
    ///
    /// # Panics
    ///
    /// See [`ClientBuilder::connect()`].
    pub fn new(endpoint_url: &str) -> Result<Self, Error> {
        ClientBuilder::default().connect(endpoint_url)
    }

    /// Read data from server.
    ///
    /// # Errors
    ///
    /// This fails when the request cannot be served.
    pub fn read(&mut self, request: ua::ReadRequest) -> Result<ua::ReadResponse, Error> {
        let response = unsafe { UA_Client_Service_read(self.0.as_mut_ptr(), request.into_inner()) };

        if response.responseHeader.serviceResult != UA_STATUSCODE_GOOD {
            return Err(Error::new(response.responseHeader.serviceResult));
        }

        Ok(ua::ReadResponse::new(response))
    }

    /// Read node ID attribute from node.
    ///
    /// # Errors
    ///
    /// This fails when the node does not exist or the node ID attribute cannot be read.
    pub fn read_node_id(&mut self, node_id: &ua::NodeId) -> Result<ua::NodeId, Error> {
        let mut output = ua::NodeId::default();
        let data_type = unsafe { &UA_TYPES[UA_TYPES_NODEID as usize] };

        let result = unsafe {
            __UA_Client_readAttribute(
                self.0.as_mut_ptr(),
                node_id.as_ptr(),
                UA_AttributeId_UA_ATTRIBUTEID_NODEID,
                output.as_mut_ptr().cast(),
                data_type,
            )
        };

        if result != UA_STATUSCODE_GOOD {
            return Err(Error::new(result));
        }

        Ok(output)
    }

    /// Read value attribute from node.
    ///
    /// # Errors
    ///
    /// This fails when the node does not exist or the value attribute cannot be read.
    pub fn read_value(&mut self, node_id: &ua::NodeId) -> Result<ua::Variant, Error> {
        let mut output = ua::Variant::default();
        let data_type = unsafe { &UA_TYPES[UA_TYPES_VARIANT as usize] };

        let result = unsafe {
            __UA_Client_readAttribute(
                self.0.as_mut_ptr(),
                node_id.as_ptr(),
                UA_AttributeId_UA_ATTRIBUTEID_VALUE,
                output.as_mut_ptr().cast(),
                data_type,
            )
        };

        if result != UA_STATUSCODE_GOOD {
            return Err(Error::new(result));
        }

        Ok(output)
    }
}
