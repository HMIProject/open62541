use anyhow::Context;
use open62541::{ua, AsyncClient};
use open62541_sys::UA_NS0ID_SERVER_SERVERSTATUS;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client =
        AsyncClient::new("opc.tcp://opcuademo.sterfive.com:26543").with_context(|| "connect")?;

    let node_id = ua::NodeId::numeric(0, UA_NS0ID_SERVER_SERVERSTATUS);

    println!("Browsing node {node_id}");

    let references = client.browse(&node_id).await.with_context(|| "browse")?;

    for reference in references {
        println!("- {}", reference.browse_name());
    }

    Ok(())
}
