use open62541::{ua, Client};
use open62541_sys::UA_AttributeId_UA_ATTRIBUTEID_VALUE;
use simple_logger::SimpleLogger;

fn main() -> Result<(), &'static str> {
    SimpleLogger::new().init().unwrap();

    let mut client = Client::new("opc.tcp://opcuademo.sterfive.com:26543").ok_or("connect")?;

    let node_id = ua::NodeId::new_numeric(0, 2256).ok_or("create node ID")?;

    println!("Reading attributes from node ID {node_id:?}");

    let read_node_id = client.read_node_id(&node_id).ok_or("read node ID")?;
    let read_value = client.read_value(&node_id).ok_or("read value")?;

    println!("node ID: {read_node_id:?}");
    println!("value: {read_value:?}");

    let nodes_to_read = ua::ReadValueId::new()
        .ok_or("create read value ID")?
        .attribute_id(UA_AttributeId_UA_ATTRIBUTEID_VALUE)
        .ok_or("set attribute ID")?
        .node_id(&read_node_id)
        .ok_or("set node ID")?;

    let request = ua::ReadRequest::new()
        .nodes_to_read(&[nodes_to_read])
        .ok_or("set nodes to read")?;

    let result = client.read(request).ok_or("read")?;

    for value in result.results().iter() {
        println!("{:?}", value.value());
    }

    Ok(())
}
