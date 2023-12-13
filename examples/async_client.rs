use std::{pin::pin, time::Duration};

use anyhow::Context;
use futures::{future, StreamExt as _};
use open62541::{ua, AsyncClient};
use open62541_sys::{
    UA_NS0ID_SERVER_SERVERSTATUS, UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_BUILDDATE,
    UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_MANUFACTURERNAME,
    UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_PRODUCTNAME, UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME,
    UA_NS0ID_SERVER_SERVERSTATUS_STARTTIME,
};
use tokio::time;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client =
        AsyncClient::new("opc.tcp://opcuademo.sterfive.com:26543").with_context(|| "connect")?;

    println!("Connected successfully");

    let node_id = ua::NodeId::numeric(0, UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME);

    let value_stream = client.value_stream(&node_id).await?;

    let mut pinned_stream = pin!(value_stream.take(5));

    while let Some(value) = pinned_stream.next().await {
        println!("{value:?}");
    }

    println!("Creating subscription");

    let subscription = client
        .create_subscription()
        .await
        .with_context(|| "create subscription")?;

    let node_id = ua::NodeId::numeric(0, UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME);

    let mut monitored_item = subscription
        .create_monitored_item(&node_id)
        .await
        .with_context(|| "monitor item")?;

    tokio::spawn(async move {
        println!("Watching for monitored item values");
        while let Some(value) = monitored_item.next().await {
            println!("{value:?}");
        }
        println!("Closed monitored item subscription");
    });

    time::sleep(Duration::from_secs(2)).await;

    drop(subscription);

    println!("Subscription dropped");

    time::sleep(Duration::from_millis(500)).await;

    println!("Reading some items");

    let builddate = ua::NodeId::numeric(0, UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_BUILDDATE);
    let manufacturername =
        ua::NodeId::numeric(0, UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_MANUFACTURERNAME);
    let productname = ua::NodeId::numeric(0, UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_PRODUCTNAME);
    let currenttime = ua::NodeId::numeric(0, UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME);
    let starttime = ua::NodeId::numeric(0, UA_NS0ID_SERVER_SERVERSTATUS_STARTTIME);

    let results = future::join_all(vec![
        client.read_value(&builddate),
        client.read_value(&manufacturername),
        client.read_value(&productname),
        client.read_value(&currenttime),
        client.read_value(&starttime),
    ])
    .await;

    println!("{results:?}");

    time::sleep(Duration::from_millis(500)).await;

    println!("Browsing node");

    let references = client
        .browse(&ua::NodeId::numeric(0, UA_NS0ID_SERVER_SERVERSTATUS))
        .await?;

    println!("{references:?}");

    time::sleep(Duration::from_millis(500)).await;

    println!("Dropping client");

    drop(client);

    time::sleep(Duration::from_millis(500)).await;

    println!("Exiting");

    Ok(())
}
