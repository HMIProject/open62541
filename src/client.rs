use std::ffi::CString;

use log::info;
use open62541_sys::{
    UA_AttributeId_UA_ATTRIBUTEID_VALUE, UA_ClientConfig_setDefault, UA_Client_connect,
    UA_Client_getConfig, __UA_Client_readAttribute, UA_STATUSCODE_GOOD, UA_TYPES, UA_TYPES_VARIANT,
};

use crate::ua;

#[allow(clippy::module_name_repetitions)]
pub struct ClientBuilder {
    client: ua::Client,
}

impl ClientBuilder {
    pub fn new() -> Option<Self> {
        let mut client = ua::Client::new()?;

        let result = unsafe {
            let config = UA_Client_getConfig(client.as_ptr());
            UA_ClientConfig_setDefault(config)
        };

        if result != UA_STATUSCODE_GOOD {
            return None;
        }

        Some(Self { client })
    }

    pub fn connect(mut self, endpoint_url: &str) -> Option<Client> {
        info!("Connecting to endpoint {endpoint_url}");

        let endpoint_url = CString::new(endpoint_url).ok()?;

        let result = unsafe { UA_Client_connect(self.client.as_ptr(), endpoint_url.as_ptr()) };

        if result != UA_STATUSCODE_GOOD {
            return None;
        }

        Some(Client {
            client: self.client,
        })
    }
}

pub struct Client {
    client: ua::Client,
}

impl Client {
    #[must_use]
    pub fn new(endpoint_url: &str) -> Option<Self> {
        let client = ClientBuilder::new()?;

        client.connect(endpoint_url)
    }

    #[must_use]
    pub fn read_value(&mut self, node_id: &ua::NodeId) -> Option<ua::Variant> {
        let attribute_id = UA_AttributeId_UA_ATTRIBUTEID_VALUE;
        let mut out = ua::Variant::new()?;
        let out_data_type = unsafe { &UA_TYPES[UA_TYPES_VARIANT as usize] };

        let result = unsafe {
            __UA_Client_readAttribute(
                self.client.as_ptr(),
                node_id.as_ptr(),
                attribute_id,
                out.as_ptr().cast(),
                out_data_type,
            )
        };

        if result != UA_STATUSCODE_GOOD {
            return None;
        }

        Some(out)
    }
}
