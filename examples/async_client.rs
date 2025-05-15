use std::{num::NonZero, time::Duration};

use anyhow::{bail, Context as _};
use futures::future;
use open62541::{
    ua, AsyncClient, ClientBuilder, DataType, MonitoredItemBuilder, SubscriptionBuilder,
};
use open62541_sys::{
    UA_NS0ID_BASEEVENTTYPE, UA_NS0ID_BASEMODELCHANGEEVENTTYPE, UA_NS0ID_SERVER,
    UA_NS0ID_SERVER_SERVERSTATUS, UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_BUILDDATE,
    UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_MANUFACTURERNAME,
    UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_PRODUCTNAME, UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME,
    UA_NS0ID_SERVER_SERVERSTATUS_STARTTIME,
};
use tokio::time;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client = ClientBuilder::default()
        .connect("opc.tcp://opcuademo.sterfive.com:26543")
        .context("connect")?
        .into_async();

    println!("Connected successfully");

    subscribe_node_value(&client).await?;

    time::sleep(Duration::from_millis(500)).await;

    subscribe_node_events(&client).await?;

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

async fn subscribe_node_value(client: &AsyncClient) -> anyhow::Result<()> {
    println!("Creating subscription");

    let subscription = client
        .create_subscription()
        .await
        .context("create subscription")?;

    let node_id = ua::NodeId::ns0(UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME);

    let mut monitored_item = subscription
        .create_monitored_item(&node_id)
        .await
        .context("monitor item")?;

    tokio::spawn(async move {
        println!("Watching for monitored item values (data changes)");
        while let Some(value) = monitored_item.next().await {
            println!("{node_id} -> {value:?}");
        }
        println!("Closed monitored item subscription");
    });

    time::sleep(Duration::from_secs(2)).await;

    drop(subscription);

    println!("Subscription dropped");

    Ok(())
}

async fn subscribe_node_events(client: &AsyncClient) -> anyhow::Result<()> {
    println!("Creating subscription");

    let subscription = client
        .create_subscription()
        .await
        .context("create subscription")?;

    let node_id = ua::NodeId::ns0(UA_NS0ID_SERVER);

    let results = MonitoredItemBuilder::new([node_id.clone()])
        .attribute_id(ua::AttributeId::EVENTNOTIFIER)
        .filter(
            ua::EventFilter::init()
                .with_select_clauses(&[
                    ua::SimpleAttributeOperand::init()
                        .with_type_definition_id(ua::NodeId::ns0(UA_NS0ID_BASEEVENTTYPE))
                        .with_browse_path(&[ua::QualifiedName::new(0, "Change")])
                        .with_attribute_id(&ua::AttributeId::VALUE),
                    ua::SimpleAttributeOperand::init()
                        .with_type_definition_id(ua::NodeId::ns0(UA_NS0ID_BASEEVENTTYPE))
                        .with_browse_path(&[ua::QualifiedName::new(0, "EventType")])
                        .with_attribute_id(&ua::AttributeId::VALUE),
                    ua::SimpleAttributeOperand::init()
                        .with_type_definition_id(ua::NodeId::ns0(UA_NS0ID_BASEEVENTTYPE))
                        .with_browse_path(&[ua::QualifiedName::new(0, "SourceNode")])
                        .with_attribute_id(&ua::AttributeId::VALUE),
                ])
                .with_where_clause(
                    ua::ContentFilter::init().with_elements(&[ua::ContentFilterElement::init()
                        .with_filter_operator(ua::FilterOperator::OFTYPE)
                        .with_filter_operands(&[ua::LiteralOperand::new(ua::Variant::scalar(
                            ua::NodeId::ns0(UA_NS0ID_BASEMODELCHANGEEVENTTYPE),
                        ))])]),
                ),
        )
        .create(&subscription)
        .await
        .context("monitor item")?;
    let Ok::<[_; 1], _>([result]) = results.try_into() else {
        bail!("expected exactly one monitored item");
    };
    let (_, mut monitored_item) = result?;

    tokio::spawn(async move {
        println!("Watching for monitored item values (events)");
        while let Some(value) = monitored_item.next().await {
            println!("{node_id} -> {value:?}");
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

    let (response, subscription) = SubscriptionBuilder::default()
        .requested_publishing_interval(Some(Duration::from_millis(100)))
        .requested_lifetime_count(5)
        .requested_max_keep_alive_count(Some(NonZero::new(1).context("non-zero value")?))
        .max_notifications_per_publish(Some(NonZero::new(3).context("non-zero value")?))
        .publishing_enabled(true)
        .priority(127)
        .create(client)
        .await
        .context("create subscription with options")?;

    println!(
        "Revised publishing interval: {:?}",
        response.revised_publishing_interval()?
    );
    println!(
        "Revised lifetime count: {}",
        response.revised_lifetime_count()
    );
    println!(
        "Revised maximum keep-alive count: {}",
        response.revised_max_keep_alive_count()
    );

    let node_ids = [
        ua::NodeId::ns0(UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME),
        ua::NodeId::ns0(UA_NS0ID_SERVER_SERVERSTATUS_STARTTIME),
    ];

    let results = MonitoredItemBuilder::new(node_ids.clone())
        .monitoring_mode(ua::MonitoringMode::REPORTING)
        .sampling_interval(Some(Duration::from_millis(100)))
        .queue_size(3)
        .discard_oldest(true)
        .create(&subscription)
        .await
        .context("monitor items")?;

    for (node_id, result) in node_ids.into_iter().zip(results) {
        let (result, mut monitored_item) = match result {
            Ok(result) => result,
            Err(err) => {
                println!("Error for {node_id}: {err:#}");
                continue;
            }
        };

        println!(
            "Revised sampling interval: {:?}",
            result.revised_sampling_interval()?
        );
        println!("Revised queue size: {}", result.revised_queue_size());

        tokio::spawn(async move {
            println!("Watching for monitored item values (data changes)");
            while let Some(value) = monitored_item.next().await {
                println!("{node_id} -> {value:?}");
            }
            println!("Closed monitored item subscription");
        });
    }

    time::sleep(Duration::from_secs(2)).await;

    drop(subscription);

    println!("Subscription with options dropped");

    Ok(())
}

async fn read_nodes(client: &AsyncClient) -> anyhow::Result<()> {
    println!("Reading some items");

    let builddate = ua::NodeId::ns0(UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_BUILDDATE);
    let manufacturername = ua::NodeId::ns0(UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_MANUFACTURERNAME);
    let productname = ua::NodeId::ns0(UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_PRODUCTNAME);
    let currenttime = ua::NodeId::ns0(UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME);
    let starttime = ua::NodeId::ns0(UA_NS0ID_SERVER_SERVERSTATUS_STARTTIME);

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
                .with_node_id(&ua::NodeId::ns0(UA_NS0ID_SERVER_SERVERSTATUS)),
        )
        .await
        .context("browse node")?;

    println!("{references:?}");

    Ok(())
}
