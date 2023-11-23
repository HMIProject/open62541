use std::ffi::CString;

use open62541_sys::{
    UA_AttributeId_UA_ATTRIBUTEID_VALUE, UA_ClientConfig_setDefault, UA_Client_connect,
    UA_Client_getConfig, __UA_Client_readAttribute, UA_STATUSCODE_GOOD, UA_TYPES, UA_TYPES_VARIANT,
};

use crate::ua;

#[allow(clippy::module_name_repetitions)]
pub struct ClientBuilder {
    client: ll::Client,
}

impl ClientBuilder {
    pub fn new() -> Option<Self> {
        let client = ll::Client::new()?;

        let result = unsafe {
            let config = UA_Client_getConfig(client.as_ptr());
            UA_ClientConfig_setDefault(config)
        };

        if result != UA_STATUSCODE_GOOD {
            return None;
        }

        Some(ClientBuilder { client })
    }

    pub fn connect(self, endpoint_url: &str) -> Option<Client> {
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
    client: ll::Client,
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
        let out = ua::Variant::new()?;
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

mod ll {
    use std::ptr::NonNull;

    use log::debug;
    use open62541_sys::{UA_Client, UA_Client_delete, UA_Client_new};

    pub struct Client(NonNull<UA_Client>);

    impl Client {
        pub fn new() -> Option<Self> {
            debug!("Creating UA_Client");

            let client = NonNull::new(unsafe { UA_Client_new() })?;

            Some(Client(client))
        }

        pub const fn as_ptr(&self) -> *mut UA_Client {
            self.0.as_ptr()
        }
    }

    impl Drop for Client {
        fn drop(&mut self) {
            debug!("Dropping UA_Client");

            unsafe { UA_Client_delete(self.0.as_ptr()) }
        }
    }
}
