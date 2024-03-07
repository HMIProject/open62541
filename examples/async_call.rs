use anyhow::{anyhow, Context as _};
use open62541::{ua, AsyncClient, DataType as _};

const CYCLE_TIME: tokio::time::Duration = tokio::time::Duration::from_millis(100);

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client = AsyncClient::new("opc.tcp://opcuademo.sterfive.com:26543", CYCLE_TIME)
        .context("connect")?;

    // `/Root/Objects/9:Simulation/9:ObjectWithMethods`
    let object_node_id = ua::NodeId::string(9, "ObjectWithMethods");
    // `/Root/Objects/9:Simulation/9:ObjectWithMethods/9:MethodNoArgs`
    let method_no_args_node_id = ua::NodeId::string(9, "MethodNoArgs");
    // `/Root/Objects/9:Simulation/9:ObjectWithMethods/9:MethodIO`
    let method_io_node_id = ua::NodeId::string(9, "MethodIO");

    let _output_arguments =
        call_method(&client, &object_node_id, &method_no_args_node_id, &[]).await?;

    println!();

    let output_arguments = call_method(
        &client,
        &object_node_id,
        &method_io_node_id,
        &[ua::Variant::init().with_scalar(&ua::UInt32::new(123))],
    )
    .await?;

    let value: i32 = output_arguments
        .ok_or(anyhow!("output arguments"))?
        .first()
        .ok_or(anyhow!("output argument"))?
        .to_scalar::<ua::Int32>()
        .ok_or(anyhow!("scalar"))?
        .value();

    println!("-> {value}");

    Ok(())
}

async fn call_method(
    client: &AsyncClient,
    object_node_id: &ua::NodeId,
    method_node_id: &ua::NodeId,
    input_arguments: &[ua::Variant],
) -> anyhow::Result<Option<Vec<ua::Variant>>> {
    println!("Calling node {method_node_id}");

    let output_arguments = client
        .call_method(object_node_id, method_node_id, input_arguments)
        .await
        .context("call")?;

    println!("-> {output_arguments:?}");

    Ok(output_arguments)
}
