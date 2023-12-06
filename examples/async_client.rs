use std::{thread, time::Duration};

use anyhow::Context;
use futures::future;
use open62541::{ua, Client};
use open62541_sys::{
    UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_BUILDDATE,
    UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_MANUFACTURERNAME,
    UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_PRODUCTNAME, UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME,
    UA_NS0ID_SERVER_SERVERSTATUS_STARTTIME,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client = Client::new("opc.tcp://opcuademo.sterfive.com:26543")
        .with_context(|| "connect")?
        .into_async();

    println!("Connected successfully");

    println!("Creating subscription");

    let subscription = client
        .create_subscription()
        .await
        .with_context(|| "create subscription")?;

    let node_id = ua::NodeId::new_numeric(0, UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME);

    let mut monitored_item = subscription
        .monitor_item(node_id)
        .await
        .with_context(|| "monitor item")?;

    thread::spawn(move || {
        println!("Watching for monitored item values");
        while let Some(value) = monitored_item.next() {
            println!("{value:?}");
        }
        println!("Closed monitored item subscription");
    });

    thread::sleep(Duration::from_secs(2));

    drop(subscription);

    println!("Subscription dropped");

    thread::sleep(Duration::from_secs(2));

    println!("Reading some items");

    let builddate = ua::NodeId::new_numeric(0, UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_BUILDDATE);
    let manufacturername =
        ua::NodeId::new_numeric(0, UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_MANUFACTURERNAME);
    let productname =
        ua::NodeId::new_numeric(0, UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_PRODUCTNAME);
    let currenttime = ua::NodeId::new_numeric(0, UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME);
    let starttime = ua::NodeId::new_numeric(0, UA_NS0ID_SERVER_SERVERSTATUS_STARTTIME);

    let results = future::join_all(vec![
        client.read_value(builddate),
        client.read_value(manufacturername),
        client.read_value(productname),
        client.read_value(currenttime),
        client.read_value(starttime),
    ])
    .await;

    println!("{results:?}");

    thread::sleep(Duration::from_secs(2));

    println!("Dropping client");

    drop(client);

    thread::sleep(Duration::from_secs(2));

    println!("Exiting");

    Ok(())
}
