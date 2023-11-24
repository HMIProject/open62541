use std::ffi::CString;

use log::info;
use open62541_sys::{
    UA_AttributeId_UA_ATTRIBUTEID_NODEID, UA_AttributeId_UA_ATTRIBUTEID_VALUE,
    UA_ClientConfig_setDefault, UA_Client_Service_read, UA_Client_connect, UA_Client_getConfig,
    __UA_Client_readAttribute, UA_STATUSCODE_GOOD, UA_TYPES, UA_TYPES_NODEID, UA_TYPES_VARIANT,
};

use crate::ua;

#[allow(clippy::module_name_repetitions)]
pub struct ClientBuilder(ua::Client);

impl ClientBuilder {
    pub fn new() -> Option<Self> {
        let mut ua_client = ua::Client::new()?;

        let result = unsafe {
            let config = UA_Client_getConfig(ua_client.as_mut_ptr());
            UA_ClientConfig_setDefault(config)
        };

        if result != UA_STATUSCODE_GOOD {
            return None;
        }

        Some(Self(ua_client))
    }

    pub fn connect(mut self, endpoint_url: &str) -> Option<Client> {
        info!("Connecting to endpoint {endpoint_url}");

        let endpoint_url = CString::new(endpoint_url).ok()?;

        let result = unsafe { UA_Client_connect(self.0.as_mut_ptr(), endpoint_url.as_ptr()) };

        if result != UA_STATUSCODE_GOOD {
            return None;
        }

        Some(Client(self.0))
    }
}

pub struct Client(ua::Client);

impl Client {
    #[must_use]
    pub fn new(endpoint_url: &str) -> Option<Self> {
        let client = ClientBuilder::new()?;

        client.connect(endpoint_url)
    }

    pub fn read(&mut self, request: ua::ReadRequest) -> Option<ua::ReadResponse> {
        let response = unsafe { UA_Client_Service_read(self.0.as_mut_ptr(), request.into_inner()) };

        if response.responseHeader.serviceResult != UA_STATUSCODE_GOOD {
            return None;
        }

        Some(ua::ReadResponse::new(response))
    }

    #[must_use]
    pub fn read_node_id(&mut self, node_id: &ua::NodeId) -> Option<ua::NodeId> {
        let mut output = ua::NodeId::new();
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
            return None;
        }

        Some(output)
    }

    #[must_use]
    pub fn read_value(&mut self, node_id: &ua::NodeId) -> Option<ua::Variant> {
        let mut output = ua::Variant::new()?;
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
            return None;
        }

        Some(output)
    }
}
