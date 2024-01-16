use std::{pin::pin, sync::Arc};

use anyhow::Context as _;
use futures::StreamExt as _;
use open62541::{ua, AsyncClient};
use open62541_sys::UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME;
use tokio::task;

const CYCLE_TIME: tokio::time::Duration = tokio::time::Duration::from_millis(100);

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client = AsyncClient::new("opc.tcp://opcuademo.sterfive.com:26543", CYCLE_TIME)
        .with_context(|| "connect")?;

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

    let value = client
        .read_value(&node_id)
        .await
        .with_context(|| "read value")?;

    println!(
        "Node {node_id} has value {:?}",
        value
            .value()
            .and_then(ua::Variant::to_scalar::<ua::DateTime>)
            .and_then(|value| value.as_datetime())
    );

    Ok(())
}

async fn watch_background(client: Arc<AsyncClient>) -> anyhow::Result<()> {
    let node_id = ua::NodeId::numeric(0, UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME);

    let subscription = client
        .create_subscription()
        .await
        .with_context(|| "create subscription")?;

    let monitored_item = subscription
        .create_monitored_item(&node_id)
        .await
        .with_context(|| "create monitored item")?;

    let mut stream = pin!(monitored_item.into_stream().take(3).enumerate());

    while let Some((index, value)) = stream.next().await {
        println!(
            "Node {node_id} emitted value #{index}: {:?}",
            value
                .value()
                .and_then(ua::Variant::to_scalar::<ua::DateTime>)
                .and_then(|value| value.as_datetime())
        );
    }

    Ok(())
}
