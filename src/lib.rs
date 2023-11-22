use open62541_sys::{UA_Client, UA_Client_delete, UA_Client_new};

pub struct Client {
    client: *mut UA_Client,
}

impl Client {
    pub fn new() -> Option<Self> {
        let client = unsafe { UA_Client_new() };

        if client.is_null() {
            return None;
        }

        Some(Client { client })
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        unsafe {
            UA_Client_delete(self.client);
        }
    }
}
