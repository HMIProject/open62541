use anyhow::{anyhow, Context as _};
use open62541::{ua, AsyncClient, DataType as _};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client =
        AsyncClient::new("opc.tcp://opcuademo.sterfive.com:26543").with_context(|| "connect")?;

    // `/Root/Objects/8:Simulation/8:ObjectWithMethods`
    let object_node_id = ua::NodeId::string(8, "ObjectWithMethods");
    // `/Root/Objects/8:Simulation/8:ObjectWithMethods/8:MethodNoArgs`
    let method_no_args_node_id = ua::NodeId::string(8, "MethodNoArgs");
    // `/Root/Objects/8:Simulation/8:ObjectWithMethods/8:MethodIO`
    let method_io_node_id = ua::NodeId::string(8, "MethodIO");

    println!("Calling node {method_no_args_node_id}");

    let input_arguments: Vec<ua::Variant> = vec![];

    let output_arguments = client
        .call_method(&object_node_id, &method_no_args_node_id, &input_arguments)
        .await
        .with_context(|| "call")?;

    println!("-> {output_arguments:?}");

    println!("Calling node {method_io_node_id}");

    let input_arguments: Vec<ua::Variant> =
        vec![ua::Variant::init().with_scalar(&ua::UInt32::new(123))];

    let output_arguments = client
        .call_method(&object_node_id, &method_io_node_id, &input_arguments)
        .await
        .with_context(|| "call")?;

    println!("-> {output_arguments:?}");

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
