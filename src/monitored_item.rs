use std::{
    cell::RefCell,
    rc::{Rc, Weak},
    sync::mpsc,
};

use crate::{ua, Error, MonitoredItemId, Subscription};

pub struct MonitoredItem {
    subscription: Weak<RefCell<Subscription>>,
    monitored_item_id: MonitoredItemId,
    rx: mpsc::Receiver<ua::DataValue>,
}

impl MonitoredItem {
    pub fn new(
        subscription: &Rc<RefCell<Subscription>>,
        node_id: &ua::NodeId,
    ) -> Result<Self, Error> {
        let (client, subscription_id) = {
            let subscription = subscription.borrow();
            (subscription.client.clone(), subscription.subscription_id)
        };

        let Some(client) = client.upgrade() else {
            todo!()
        };

        let (tx, rx) = mpsc::channel::<ua::DataValue>();

        let response = client.borrow_mut().create_data_change(
            subscription_id,
            ua::MonitoredItemCreateRequest::init_node_id(node_id.clone()),
            move |value| {
                let _unused = tx.send(value);
            },
        )?;

        let monitored_item = Self {
            subscription: Rc::downgrade(subscription),
            monitored_item_id: response.monitored_item_id(),
            rx,
        };

        Ok(monitored_item)
    }

    pub fn rx(&self) -> &mpsc::Receiver<ua::DataValue> {
        &self.rx
    }
}

impl Drop for MonitoredItem {
    fn drop(&mut self) {
        // TODO: Remove monitored item handler.
    }
}
