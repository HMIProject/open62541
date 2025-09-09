use std::{sync::Arc, time::Duration};

use anyhow::{anyhow, bail, Context as _};
use open62541::{
    create_monitored_items_callback, ua, AsyncClient, AsyncSubscription,
    MonitoredItemCreateRequestBuilder, MonitoredItemHandle, MonitoredItemValue,
};
use open62541_sys::{
    UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_PRODUCTNAME, UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME,
};
use rand::Rng as _;
use tokio::{
    sync::watch,
    time::{self, error::Elapsed},
};

#[expect(
    clippy::panic_in_result_fn,
    reason = "Unexpected and inconsistent results are not handled."
)]
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

    let node_ids = [&float_node_id, &date_time_node_id, &string_node_id];
    let handles_rxs =
        watch_node_values(&subscription, node_ids.iter().copied().map(Clone::clone)).await?;
    assert_eq!(node_ids.len(), handles_rxs.len());

    let tasks = node_ids
        .into_iter()
        .zip(handles_rxs)
        .map(|(node_id, (handle, rx))| {
            tokio::spawn(monitor_background_task(node_id.clone(), handle, rx))
        })
        .chain([tokio::spawn(write_background(
            Arc::clone(&client),
            float_node_id.clone(),
        ))]);
    for task in tasks {
        task.await??;
    }

    Ok(())
}

fn create_value_callback_rx() -> (
    impl FnMut(MonitoredItemValue) + 'static,
    watch::Receiver<Option<MonitoredItemValue>>,
) {
    let (tx, rx) = watch::channel(None);
    let callback = move |new_value| {
        tx.send_if_modified(|value| {
            let new_value = Some(new_value);
            if *value == new_value {
                // Deduplicate equal values.
                return false;
            }
            *value = new_value;
            true
        });
    };
    (callback, rx)
}

async fn watch_node_values(
    subscription: &AsyncSubscription,
    node_ids: impl IntoIterator<Item = ua::NodeId>,
) -> anyhow::Result<
    Vec<(
        MonitoredItemHandle,
        watch::Receiver<Option<MonitoredItemValue>>,
    )>,
> {
    let Some(client) = subscription.client().upgrade() else {
        bail!("not subscribed");
    };
    let subscription_id = subscription.subscription_id();

    let create_request_builder = MonitoredItemCreateRequestBuilder::new(node_ids);
    let node_ids = create_request_builder.node_ids().to_vec();
    let mut rxs = Vec::with_capacity(node_ids.len());
    let create_responses = create_monitored_items_callback(
        &client,
        subscription_id,
        create_request_builder,
        |index: usize| {
            assert_eq!(index, rxs.len());
            let (callback, rx) = create_value_callback_rx();
            rxs.push(rx);
            callback
        },
    )
    .await
    .with_context(|| {
        format!(
            "create monitored {node_count} item(s)",
            node_count = node_ids.len()
        )
    })?;
    assert_eq!(create_responses.len(), node_ids.len());
    assert_eq!(rxs.len(), node_ids.len());

    let handles_rxs = node_ids
        .into_iter()
        .zip(create_responses.into_iter().zip(rxs))
        .map(|(node_id, (response, rx))| {
            response
                .map(|(_create_result, handle)| (handle, rx))
                .map_err(|err| anyhow!("create monitored item for node {node_id}: {err:#}"))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    Ok(handles_rxs)
}

async fn monitor_background_task(
    node_id: ua::NodeId,
    mut handle: MonitoredItemHandle,
    mut rx: watch::Receiver<Option<MonitoredItemValue>>,
) -> anyhow::Result<()> {
    let task = {
        let node_id = node_id.clone();
        async move {
            while let Ok(()) = rx.changed().await {
                // The shared value should only borrowed for a short period of time!
                // Otherwise the client event loop will be blocked.
                let value_borrowed_ref = rx.borrow_and_update();
                let Some(value_borrowed) = &*value_borrowed_ref else {
                    // Initial value is unset.
                    continue;
                };
                let Some(value) = value_borrowed.value() else {
                    bail!("received unexpected value from node {node_id}: {value_borrowed:?}");
                };
                println!("Received value from node {node_id}: {value:?}");
            }
            Ok(())
        }
    };

    tokio::spawn(tokio::time::timeout(Duration::from_secs(2), task))
        .await?
        // Ignore timeout error because it is actually expected.
        .unwrap_or_else(|_: Elapsed| Ok(()))?;

    println!("Deleting monitored item for node {node_id} on server");
    handle.delete_async();

    Ok(())
}

async fn write_background(client: Arc<AsyncClient>, node_id: ua::NodeId) -> anyhow::Result<()> {
    let value = rand::rng().random_range(0.0..100.0);

    time::sleep(Duration::from_secs(1)).await;

    println!("Writing {value} to node {node_id}");

    let value = ua::DataValue::new(ua::Variant::scalar(ua::Float::new(value)));

    client
        .write_value(&node_id, &value)
        .await
        .context("write value")?;

    Ok(())
}
