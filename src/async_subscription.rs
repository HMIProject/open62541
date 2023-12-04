use std::sync::{Mutex, Weak};

use crate::ua;

pub struct AsyncSubscription {
    pub(crate) client: Weak<Mutex<ua::Client>>,
}

impl Drop for AsyncSubscription {
    fn drop(&mut self) {
        if let Some(client) = self.client.upgrade() {
            if let Ok(_client) = client.lock() {
                // TODO: Delete subscription.
            }
        }
    }
}
