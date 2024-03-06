use anyhow::Context as _;
use open62541::{ua, AsyncClient, DataType as _};
use rand::Rng as _;

const CYCLE_TIME: tokio::time::Duration = tokio::time::Duration::from_millis(100);

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

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client = AsyncClient::new("opc.tcp://opcuademo.sterfive.com:26543", CYCLE_TIME)
        .context("connect")?;

    // `/Root/Objects/2:DeviceSet/1:CoffeeMachine/1:Espresso/7:BeverageSize`
    let node_id = ua::NodeId::numeric(1, 1074);

    println!("Reading node {node_id}");

    let attribute_values = client.read_attributes(&node_id, &ATTRIBUTE_IDS).await?;

    for (attribute_id, value) in ATTRIBUTE_IDS.iter().zip(attribute_values.iter()) {
        match value {
            Ok(value) => match value.value() {
                Some(value) => println!("- {attribute_id} -> {value:?}"),
                None => println!("- {attribute_id} -> (no value)"),
            },
            Err(err) => println!("- {attribute_id} -> {err}"),
        }
    }

    let value = client.read_value(&node_id).await.context("read")?;

    println!("-> {value:?}");

    let value = rand::thread_rng().gen_range(0.0..100.0);

    println!("Writing {value} to node {node_id}");

    client
        .write_value(
            &node_id,
            &ua::DataValue::init()
                .with_value(&ua::Variant::init().with_scalar(&ua::Float::new(value))),
        )
        .await
        .context("write")?;

    println!("Reading node {node_id}");

    let value = client.read_value(&node_id).await.context("read")?;

    println!("-> {value:?}");

    Ok(())
}
