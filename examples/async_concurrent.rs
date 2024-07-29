use std::sync::Arc;

use anyhow::{anyhow, Context as _};
use open62541::{ua, AsyncClient};
use open62541_sys::{UA_NS0ID_SERVER, UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME};
use tokio::task::JoinSet;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client =
        Arc::new(AsyncClient::new("opc.tcp://opcuademo.sterfive.com:26543").context("connect")?);

    let mut tasks = JoinSet::new();

    // Spawn many tasks in order to trigger race conditions in multi-threaded use of open62541.
    //
    for _ in 0..100 {
        let client = Arc::clone(&client);

        tasks.spawn(async move {
            let (references, _) = client
                .browse(
                    &ua::BrowseDescription::default()
                        .with_node_id(&ua::NodeId::ns0(UA_NS0ID_SERVER)),
                )
                .await
                .context("browse")?;
            println!("References: {}", references.len());

            let value = client
                .read_value(&ua::NodeId::ns0(UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME))
                .await
                .context("read")?;
            let value = value.value().ok_or(anyhow!("no value"))?.to_value();
            println!("Value: {value:?}");

            Ok::<_, anyhow::Error>(())
        });
    }

    while let Some(result) = tasks.join_next().await {
        result??;
    }

    Ok(())
}
