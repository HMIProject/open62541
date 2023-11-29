use std::{
    cell::RefCell,
    rc::Rc,
    sync::mpsc::{self, RecvTimeoutError},
    time::Duration,
};

use anyhow::Context;
use open62541::{ua, Client, Error, MonitoredItem, Subscription};
use open62541_sys::{
    UA_AttributeId_UA_ATTRIBUTEID_VALUE, UA_NS0ID_SERVER_SERVERSTATUS,
    UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_BUILDDATE,
    UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_MANUFACTURERNAME,
    UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_PRODUCTNAME, UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME,
    UA_NS0ID_SERVER_SERVERSTATUS_STARTTIME,
};
use simple_logger::SimpleLogger;

fn main() -> anyhow::Result<()> {
    SimpleLogger::new().init().unwrap();

    let client = Rc::new(RefCell::new(
        Client::new("opc.tcp://opcuademo.sterfive.com:26543").with_context(|| "connect")?,
    ));

    read_single_value(
        &mut client.borrow_mut(),
        &ua::NodeId::new_numeric(0, UA_NS0ID_SERVER_SERVERSTATUS),
    )?;

    read_multiple_values(
        &mut client.borrow_mut(),
        &[
            ua::NodeId::new_numeric(0, UA_NS0ID_SERVER_SERVERSTATUS_STARTTIME),
            ua::NodeId::new_numeric(0, UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME),
            ua::NodeId::new_numeric(0, UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_PRODUCTNAME),
            ua::NodeId::new_numeric(0, UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_MANUFACTURERNAME),
            ua::NodeId::new_numeric(0, UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_BUILDDATE),
        ],
    )?;

    subscribe_single_value_raw(
        &mut client.borrow_mut(),
        &ua::NodeId::new_numeric(0, UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME),
    )?;

    subscribe_single_value_helpers(
        &client,
        &ua::NodeId::new_numeric(0, UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME),
    )?;

    Ok(())
}

fn read_single_value(client: &mut Client, node_id: &ua::NodeId) -> anyhow::Result<()> {
    let value = client.read_value(&node_id).with_context(|| "read value")?;

    println!("Got value from {node_id}: {value}");

    Ok(())
}

fn read_multiple_values(client: &mut Client, node_ids: &[ua::NodeId]) -> anyhow::Result<()> {
    let nodes_to_read: Vec<_> = node_ids
        .iter()
        .map(|node_id| {
            ua::ReadValueId::init()
                .with_attribute_id(UA_AttributeId_UA_ATTRIBUTEID_VALUE)
                .with_node_id(node_id)
        })
        .collect();

    let request = ua::ReadRequest::init().with_nodes_to_read(&nodes_to_read);

    let response = client.read(request).with_context(|| "read from client")?;
    let results = response.results().with_context(|| "get read results")?;
    let results = results.as_slice();

    println!("Got {} values from node IDs:", results.len());

    for (node_id, value) in node_ids.iter().zip(results.iter()) {
        println!("- {node_id} -> {:?}", value.value());
    }

    Ok(())
}

fn subscribe_single_value_raw(client: &mut Client, node_id: &ua::NodeId) -> anyhow::Result<()> {
    let create_req = ua::CreateSubscriptionRequest::default();

    println!("CreateSubscription request: {create_req:?}");

    let create_res = client
        .create_subscription(create_req)
        .with_context(|| "create subscription")?;

    println!("CreateSubscription response: {create_res:?}");

    {
        let mon_req = ua::MonitoredItemCreateRequest::init_node_id(node_id.clone());

        println!("MonitoredItemCreate request: {mon_req:?}");

        let mon_res =
            client.create_data_change(create_res.subscription_id(), mon_req, |value| {
                println!("MonitoredItemCreate callback: {value:?}")
            })?;

        println!("MonitoredItemCreate result: {mon_res:?}");
    }

    client.run_iterate().with_context(|| "run iterate")?;
    client.run_iterate().with_context(|| "run iterate")?;
    client.run_iterate().with_context(|| "run iterate")?;

    let delete_req = ua::DeleteSubscriptionsRequest::init()
        .with_subscription_ids(&[create_res.subscription_id()]);

    println!("DeleteSubscriptions request: {delete_req:?}");

    let delete_res = client
        .delete_subscriptions(delete_req)
        .with_context(|| "delete subscriptions")?;

    println!("DeleteSubscriptions response: {delete_res:?}");

    Ok(())
}

fn subscribe_single_value_helpers(
    client: &Rc<RefCell<Client>>,
    node_id: &ua::NodeId,
) -> anyhow::Result<()> {
    let subscription = Subscription::new(client)?;

    let monitored_item = MonitoredItem::new(&subscription, node_id)?;

    let mut count = 0;

    while let Some(value) = recv_with_iterate(client, monitored_item.rx())? {
        println!("{node_id} -> {value}");
        count += 1;
        if count >= 4 {
            break;
        }
    }

    Ok(())
}

fn recv_with_iterate(
    client: &Rc<RefCell<Client>>,
    rx: &mpsc::Receiver<ua::DataValue>,
) -> Result<Option<ua::DataValue>, Error> {
    loop {
        match rx.recv_timeout(Duration::from_millis(50)) {
            Ok(value) => {
                return Ok(Some(value));
            }
            Err(RecvTimeoutError::Disconnected) => {
                return Ok(None);
            }
            Err(RecvTimeoutError::Timeout) => {
                client.borrow_mut().run_iterate()?;
                continue;
            }
        };
    }
}
