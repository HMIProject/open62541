use open62541::{ua, Client};
use simple_logger::SimpleLogger;

fn main() -> Result<(), &'static str> {
    SimpleLogger::new().init().unwrap();

    let mut client = Client::new("opc.tcp://opcuademo.sterfive.com:26543").ok_or("connect")?;

    let node_id = ua::NodeId::new_numeric(0, 2258).ok_or("create NodeId")?;

    println!("Reading node ID {node_id:?}");

    let value = client.read_value(&node_id).ok_or("read CurrentTime")?;

    println!("CurrentTime: {value:?}");

    Ok(())
}
