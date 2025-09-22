use std::{future::Future, sync::Arc, time::Duration};

use anyhow::Context as _;
use futures::future::try_join3;
use open62541::{
    ua, AsyncClient, AsyncSubscription, DataValue, MonitoredItemCreateRequestBuilder,
    MonitoredItemValue,
};
use open62541_sys::{
    UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_PRODUCTNAME, UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME,
};
use rand::Rng as _;

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

    let monitor_typed_task = monitor_background_typed(
        Arc::clone(&subscription),
        [
            float_node_id.clone(),
            date_time_node_id.clone(),
            string_node_id.clone(),
        ],
    )
    .await?;
    let monitor_untyped_task = monitor_background_untyped(
        Arc::clone(&subscription),
        [float_node_id.clone(), date_time_node_id, string_node_id],
    )
    .await?;
    let write_task = write_background(client, float_node_id);

    try_join3(monitor_typed_task, monitor_untyped_task, write_task).await?;

    Ok(())
}

async fn monitor_background_typed(
    subscription: Arc<AsyncSubscription>,
    node_ids: impl IntoIterator<Item = ua::NodeId>,
) -> anyhow::Result<impl Future<Output = anyhow::Result<()>> + Send> {
    let create_request_builder =
        MonitoredItemCreateRequestBuilder::new(node_ids).attribute(ua::AttributeId::VALUE_T);

    let create_value_callback_fn = |index| {
        move |value: DataValue<ua::Variant>| {
            println!("Received value for monitored item with index = {index}: {value:?}");
        }
    };

    let monitored_item_results = subscription
        .create_monitored_items_callback(create_request_builder, create_value_callback_fn)
        .await
        .context("create monitored items")?;
    let monitored_item_handles = monitored_item_results
        .into_iter()
        .map(|res| res.map(|(_, handle)| handle))
        .collect::<Result<Vec<_>, _>>()
        .context("create monitored item")?;

    let task = async move {
        // The task must take ownership of monitored_item_handles to keep them alive.
        println!(
            "Monitoring {count} item(s)",
            count = monitored_item_handles.len()
        );
        tokio::time::sleep(Duration::from_secs(2)).await;
        Ok(())
    };

    Ok(task)
}

async fn monitor_background_untyped(
    subscription: Arc<AsyncSubscription>,
    node_ids: impl IntoIterator<Item = ua::NodeId>,
) -> anyhow::Result<impl Future<Output = anyhow::Result<()>> + Send> {
    // Use attribute_id() to check the behavior for an `Unknown` monitored item kind
    // whose value type is unknown at compile time.
    let create_request_builder =
        MonitoredItemCreateRequestBuilder::new(node_ids).attribute_id(ua::AttributeId::VALUE);

    let create_value_callback_fn = |index| {
        move |value: MonitoredItemValue| {
            println!("Received value for monitored item with index = {index}: {value:?}");
        }
    };

    let monitored_item_results = subscription
        .create_monitored_items_callback(create_request_builder, create_value_callback_fn)
        .await
        .context("create monitored items")?;
    let monitored_item_handles = monitored_item_results
        .into_iter()
        .map(|res| res.map(|(_, handle)| handle))
        .collect::<Result<Vec<_>, _>>()
        .context("create monitored item")?;

    let task = async move {
        // The task must take ownership of monitored_item_handles to keep them alive.
        println!(
            "Monitoring {count} item(s)",
            count = monitored_item_handles.len()
        );
        tokio::time::sleep(Duration::from_secs(2)).await;
        Ok(())
    };

    Ok(task)
}

fn write_background(
    client: Arc<AsyncClient>,
    node_id: ua::NodeId,
) -> impl Future<Output = anyhow::Result<()>> + Send {
    let value = rand::rng().random_range(0.0..100.0);

    async move {
        tokio::time::sleep(Duration::from_secs(1)).await;

        println!("Writing {value} to node {node_id}");

        let value = ua::DataValue::new(ua::Variant::scalar(ua::Float::new(value)));

        client
            .write_value(&node_id, &value)
            .await
            .context("write value")?;

        Ok(())
    }
}
