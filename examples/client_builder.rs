use anyhow::Context as _;
use open62541::{ClientBuilder, DataType as _, ua};

fn main() -> anyhow::Result<()> {
    env_logger::init();

    println!("Building client");

    let client_description = ua::ApplicationDescription::init()
        .with_application_uri("https://crates.io/crates/open62541")
        .with_product_uri("https://crates.io/crates/open62541")
        .with_application_name("en-US", "open62541")
        .with_application_type(ua::ApplicationType::CLIENT);

    println!("Client description: {client_description:?}");

    let client = ClientBuilder::default()
        .client_description(client_description)
        .connect("opc.tcp://opcuademo.sterfive.com:26543")
        .context("connect")?;

    println!("Connected successfully");

    println!("Disconnecting client");

    client.disconnect();

    println!("Exiting");

    Ok(())
}
