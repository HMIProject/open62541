use std::{num::NonZero, time::Duration};

use anyhow::{bail, Context as _};
use futures::future;
use open62541::{ua, AsyncClient, MonitoredItemBuilder, SubscriptionBuilder};
use open62541_sys::{
    UA_NS0ID_SERVER_SERVERSTATUS, UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_BUILDDATE,
    UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_MANUFACTURERNAME,
    UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_PRODUCTNAME, UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME,
    UA_NS0ID_SERVER_SERVERSTATUS_STARTTIME,
};
use tokio::time;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client = AsyncClient::new("opc.tcp://opcuademo.sterfive.com:26543").context("connect")?;

    println!("Connected successfully");

    subscribe_node(&client).await?;

    time::sleep(Duration::from_millis(500)).await;

    subscribe_node_with_options(&client).await?;

    time::sleep(Duration::from_millis(500)).await;

    read_nodes(&client).await?;

    time::sleep(Duration::from_millis(500)).await;

    browse_node(&client).await?;

    time::sleep(Duration::from_millis(500)).await;

    println!("Disconnecting client");

    client.disconnect().await;

    time::sleep(Duration::from_millis(500)).await;

    println!("Exiting");

    Ok(())
}

async fn subscribe_node(client: &AsyncClient) -> anyhow::Result<()> {
    println!("Creating subscription");

    let subscription = client
        .create_subscription()
        .await
        .context("create subscription")?;

    let node_id = ua::NodeId::numeric(0, UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME);

    let mut monitored_item = subscription
        .create_monitored_item(&node_id)
        .await
        .context("monitor item")?;

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

    Ok(())
}

async fn subscribe_node_with_options(client: &AsyncClient) -> anyhow::Result<()> {
    println!("Creating subscription with options");

    let subscription = SubscriptionBuilder::default()
        .requested_publishing_interval(Some(Duration::from_millis(100)))
        .requested_lifetime_count(5)
        .requested_max_keep_alive_count(Some(NonZero::new(1).context("non-zero value")?))
        .max_notifications_per_publish(Some(NonZero::new(3).context("non-zero value")?))
        .publishing_enabled(true)
        .priority(127)
        .create(client)
        .await
        .context("create subscription with options")?;

    let node_id = ua::NodeId::numeric(0, UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME);

    let monitored_items = MonitoredItemBuilder::new([node_id])
        .monitoring_mode(ua::MonitoringMode::REPORTING)
        .sampling_interval(Some(Duration::from_millis(100)))
        .queue_size(3)
        .discard_oldest(true)
        .create(&subscription)
        .await
        .context("monitor items")?;

    let Some(mut monitored_item) = monitored_items.into_iter().next() else {
        bail!("expected monitored item");
    };

    tokio::spawn(async move {
        println!("Watching for monitored item values");
        while let Some(value) = monitored_item.next().await {
            println!("{value:?}");
        }
        println!("Closed monitored item subscription");
    });

    time::sleep(Duration::from_secs(2)).await;

    drop(subscription);

    println!("Subscription with options dropped");

    Ok(())
}

async fn read_nodes(client: &AsyncClient) -> anyhow::Result<()> {
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

    Ok(())
}

async fn browse_node(client: &AsyncClient) -> anyhow::Result<()> {
    println!("Browsing node");

    let (references, _) = client
        .browse(
            &ua::BrowseDescription::default()
                .with_node_id(&ua::NodeId::numeric(0, UA_NS0ID_SERVER_SERVERSTATUS)),
        )
        .await
        .context("browse node")?;

    println!("{references:?}");

    Ok(())
}
