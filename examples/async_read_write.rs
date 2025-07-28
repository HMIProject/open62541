use std::fmt::Debug;

use anyhow::Context as _;
use open62541::{ua, AsyncClient, DataType};
use rand::Rng as _;

const ATTRIBUTE_IDS: [ua::AttributeId; 11] = [
    ua::AttributeId::NODEID,
    ua::AttributeId::NODECLASS,
    ua::AttributeId::BROWSENAME,
    ua::AttributeId::DISPLAYNAME,
    ua::AttributeId::VALUE,
    ua::AttributeId::DATATYPE,
    ua::AttributeId::VALUERANK,
    ua::AttributeId::ARRAYDIMENSIONS,
    ua::AttributeId::ACCESSLEVEL,
    ua::AttributeId::USERACCESSLEVEL,
    ua::AttributeId::DATATYPEDEFINITION,
];

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client = AsyncClient::new("opc.tcp://opcuademo.sterfive.com:26543").context("connect")?;

    // `/Root/Objects/2:DeviceSet/1:CoffeeMachine/1:Espresso/7:BeverageSize`
    let float_node_id = ua::NodeId::numeric(1, 1074);
    // `/Root/Objects/2:DeviceSet/1:RFIDScanner/4:ScanActive`
    let bool_node_id = ua::NodeId::string(1, "RFIDScanner-ScanActive");

    read_attributes(&client, &float_node_id).await?;

    println!();

    let value = rand::rng().random_range(0.0..100.0);
    write_value(&client, &float_node_id, &ua::Float::new(value)).await?;

    println!();

    let value = rand::rng().random_bool(0.5);
    write_value(&client, &bool_node_id, &ua::Boolean::new(value)).await?;

    Ok(())
}

async fn read_attributes(client: &AsyncClient, node_id: &ua::NodeId) -> anyhow::Result<()> {
    println!("Attributes of {node_id}");

    let attribute_values = client.read_attributes(node_id, &ATTRIBUTE_IDS).await?;

    for (attribute_id, value) in ATTRIBUTE_IDS.iter().zip(attribute_values.iter()) {
        println!("- {attribute_id} -> {value:?}");
    }

    Ok(())
}

async fn write_value<T: DataType + Debug>(
    client: &AsyncClient,
    node_id: &ua::NodeId,
    value: &T,
) -> anyhow::Result<()> {
    let current_value = client.read_value(node_id).await.context("read")?;
    println!("Current value of {node_id}: {current_value:?}");

    println!("Writing {value:?} to node {node_id}");
    client
        .write_value(
            node_id,
            &ua::DataValue::new(ua::Variant::scalar(value.clone())),
        )
        .await
        .context("write")?;

    let updated_value = client.read_value(node_id).await.context("read")?;
    println!("Updated value of {node_id}: {updated_value:?}");

    Ok(())
}
