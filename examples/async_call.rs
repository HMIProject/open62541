use anyhow::{anyhow, bail, Context as _};
use open62541::{ua, AsyncClient};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client =
        AsyncClient::new("opc.tcp://opcuademo.sterfive.com:26543").with_context(|| "connect")?;

    // `/Root/Objects/1:Boiler#1/1:Simulation`
    let simulation_node_id = ua::NodeId::numeric(1, 1801);
    // `/Root/Objects/1:Boiler#1/1:Simulation/Reset`
    let reset_node_id = ua::NodeId::numeric(1, 1803);
    // `/Root/Objects/1:Boiler#1/1:Simulation/Start`
    let start_node_id = ua::NodeId::numeric(1, 1804);
    // `/Root/Objects/1:Boiler#1/1:Simulation/Suspend`
    let suspend_node_id = ua::NodeId::numeric(1, 1805);
    // `/Root/Objects/1:Boiler#1/1:Simulation/CurrentState`
    let current_state_node_id = ua::NodeId::numeric(1, 1807);

    println!("Reading current state from {current_state_node_id}");

    let current_state = client
        .read_value(&current_state_node_id)
        .await
        .with_context(|| "read")?;

    println!("-> {current_state:?}");

    let current_state = current_state
        .value()
        .ok_or(anyhow!("get value"))?
        .to_scalar::<ua::LocalizedText>()
        .ok_or(anyhow!("get scalar"))?
        .text()
        .to_string()
        .into_owned();

    let method_node_id = if current_state == "Halted" {
        reset_node_id
    } else if current_state == "Ready" || current_state == "Suspended" {
        start_node_id
    } else if current_state == "Running" {
        suspend_node_id
    } else {
        bail!("unknown state");
    };

    println!("Calling node {method_node_id}");

    let input_arguments: Vec<ua::Variant> = vec![];

    let output_arguments = client
        .call_method(&simulation_node_id, &method_node_id, &input_arguments)
        .await
        .with_context(|| "call")?;

    println!("-> {output_arguments:?}");

    println!("Reading current state from {current_state_node_id}");

    let current_state = client
        .read_value(&current_state_node_id)
        .await
        .with_context(|| "read")?;

    println!("-> {current_state:?}");

    Ok(())
}
