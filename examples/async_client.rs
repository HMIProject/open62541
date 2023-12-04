use std::{thread, time::Duration};

use anyhow::Context;
use open62541::{ua, Client};
use open62541_sys::{
    UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_PRODUCTNAME, UA_NS0ID_SERVER_SERVERSTATUS_STARTTIME,
};
use simple_logger::SimpleLogger;
use tokio::join;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    SimpleLogger::new().init().unwrap();

    let client = Client::new("opc.tcp://opcuademo.sterfive.com:26543")
        .with_context(|| "connect")?
        .into_async();

    let productname =
        ua::NodeId::new_numeric(0, UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_PRODUCTNAME);
    let starttime = ua::NodeId::new_numeric(0, UA_NS0ID_SERVER_SERVERSTATUS_STARTTIME);

    let results = join!(client.read_value(productname), client.read_value(starttime));

    println!("{results:?}");

    drop(client);

    thread::sleep(Duration::from_millis(1000));

    Ok(())
}
