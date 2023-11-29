use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{ua, Client, Error, SubscriptionId};

pub struct Subscription {
    pub(crate) client: Weak<RefCell<Client>>,
    pub(crate) subscription_id: SubscriptionId,
}

impl Subscription {
    pub fn new(client: &Rc<RefCell<Client>>) -> Result<Rc<RefCell<Self>>, Error> {
        let response = client
            .borrow_mut()
            .create_subscription(ua::CreateSubscriptionRequest::default())?;

        let subscription = Self {
            client: Rc::downgrade(client),
            subscription_id: response.subscription_id(),
        };

        Ok(Rc::new(RefCell::new(subscription)))
    }
}

impl Drop for Subscription {
    fn drop(&mut self) {
        let Some(client) = self.client.upgrade() else {
            return;
        };

        let _unused = client.borrow_mut().delete_subscriptions(
            ua::DeleteSubscriptionsRequest::init().with_subscription_ids(&[self.subscription_id]),
        );
    }
}
