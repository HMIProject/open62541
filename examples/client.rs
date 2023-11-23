use open62541::{ua, Client};
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

    Ok(())
}
