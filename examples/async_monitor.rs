use std::{sync::Arc, time::Duration};

use anyhow::Context as _;
use open62541::{ua, AsyncClient, AsyncSubscription};
use open62541_sys::{
    UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_PRODUCTNAME, UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME,
};
use rand::Rng as _;
use tokio::time::{self, error::Elapsed};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    println!("Connecting client");

    let client =
        Arc::new(AsyncClient::new("opc.tcp://opcuademo.sterfive.com:26543").context("connect")?);

    println!("Creating subscription");

    let subscription = Arc::new(
        client
            .create_subscription()
            .await
            .context("create first subscription")?,
    );

    // `/Root/Objects/2:DeviceSet/1:CoffeeMachine/1:Espresso/7:BeverageSize`
    let float_node_id = ua::NodeId::numeric(1, 1074);
    // `/Root/Objects/Server/ServerStatus/CurrentTime`
    let date_time_node_id = ua::NodeId::numeric(0, UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME);
    // `/Root/Objects/Server/ServerStatus/BuildInfo/ProductName`
    let string_node_id = ua::NodeId::numeric(0, UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_PRODUCTNAME);

    let tasks = vec![
        tokio::spawn(monitor_background(
            Arc::clone(&subscription),
            float_node_id.clone(),
        )),
        tokio::spawn(monitor_background(
            Arc::clone(&subscription),
            date_time_node_id,
        )),
        tokio::spawn(monitor_background(
            Arc::clone(&subscription),
            string_node_id,
        )),
        tokio::spawn(write_background(Arc::clone(&client), float_node_id)),
    ];

    for task in tasks {
        task.await??;
    }

    Ok(())
}

async fn monitor_background(
    subscription: Arc<AsyncSubscription>,
    node_id: ua::NodeId,
) -> anyhow::Result<()> {
    println!("Creating monitored item for node {node_id}");

    let mut monitored_item = subscription
        .create_monitored_item(&node_id)
        .await
        .context("create monitored item")?;

    let task = {
        let node_id = node_id.clone();
        async move {
            while let Some(value) = monitored_item.next().await {
                let value = value.value();
                println!("Received value from node {node_id}: {value:?}");
            }
            Ok::<(), anyhow::Error>(())
        }
    };

    tokio::spawn(tokio::time::timeout(Duration::from_secs(2), task))
        .await?
        // Ignore timeout error because it is actually expected.
        .unwrap_or_else(|_: Elapsed| Ok(()))?;

    println!("Deleting monitored item for node {node_id}");

    Ok(())
}

async fn write_background(client: Arc<AsyncClient>, node_id: ua::NodeId) -> anyhow::Result<()> {
    let value = rand::thread_rng().gen_range(0.0..100.0);

    time::sleep(Duration::from_secs(1)).await;

    println!("Writing {value} to node {node_id}");

    let value = ua::DataValue::new(ua::Variant::scalar(ua::Float::new(value)));

    client
        .write_value(&node_id, &value)
        .await
        .context("write value")?;

    Ok(())
}
