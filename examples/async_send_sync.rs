use std::{pin::pin, sync::Arc};

use anyhow::Context as _;
use futures::StreamExt as _;
use open62541::{AsyncClient, ua};
use open62541_sys::UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME;
use tokio::task;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client = AsyncClient::new("opc.tcp://opcuademo.sterfive.com:26543").context("connect")?;

    println!("Client connected successfully");

    let client = Arc::new(client);

    let tasks = vec![
        task::spawn(read_background(Arc::clone(&client))),
        task::spawn(read_background(Arc::clone(&client))),
        task::spawn(read_background(Arc::clone(&client))),
        task::spawn(watch_background(Arc::clone(&client))),
        task::spawn(watch_background(Arc::clone(&client))),
    ];

    for task in tasks {
        task.await??;
    }

    Ok(())
}

async fn read_background(client: Arc<AsyncClient>) -> anyhow::Result<()> {
    let node_id = ua::NodeId::numeric(0, UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME);

    let value = client.read_value(&node_id).await.context("read value")?;

    println!(
        "Node {node_id} has value {:?}",
        value
            .scalar_value()
            .context("get value")?
            .as_scalar()
            .and_then(ua::DateTime::to_utc)
    );

    Ok(())
}

async fn watch_background(client: Arc<AsyncClient>) -> anyhow::Result<()> {
    let node_id = ua::NodeId::numeric(0, UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME);

    let subscription = client
        .create_subscription()
        .await
        .context("create subscription")?;

    let monitored_item = subscription
        .create_monitored_item(&node_id)
        .await
        .context("create monitored item")?;

    let mut stream = pin!(monitored_item.take(3).enumerate());

    while let Some((index, value)) = stream.next().await {
        let value = value
            .value()
            .and_then(ua::Variant::as_scalar)
            .and_then(ua::DateTime::to_utc);
        println!("Node {node_id} emitted value #{index}: {value:?}");
    }

    Ok(())
}
