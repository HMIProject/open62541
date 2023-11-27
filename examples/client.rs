use anyhow::Context;
use open62541::{ua, Client};
use open62541_sys::UA_AttributeId_UA_ATTRIBUTEID_VALUE;
use simple_logger::SimpleLogger;

fn main() -> anyhow::Result<()> {
    SimpleLogger::new().init().unwrap();

    let mut client =
        Client::new("opc.tcp://opcuademo.sterfive.com:26543").with_context(|| "connect")?;

    let node_id = ua::NodeId::new_numeric(0, 2256);

    println!("Reading attributes from node ID {node_id:?}");

    let read_node_id = client
        .read_node_id(&node_id)
        .with_context(|| "read node ID")?;
    let read_value = client.read_value(&node_id).with_context(|| "read value")?;

    println!("node ID: {read_node_id:?}");
    println!("value: {read_value:?}");

    let nodes_to_read = ua::ReadValueId::default()
        .attribute_id(UA_AttributeId_UA_ATTRIBUTEID_VALUE)
        .node_id(&read_node_id);

    let request = ua::ReadRequest::default()
        .nodes_to_read(&[nodes_to_read])
        .with_context(|| "set nodes to read")?;

    let result = client.read(request).with_context(|| "read")?;

    for value in result.results().iter() {
        println!("{:?}", value.value());
    }

    Ok(())
}
