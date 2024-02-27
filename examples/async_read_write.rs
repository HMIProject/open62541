use std::collections::HashMap;

use anyhow::Context as _;
use open62541::{ua, AsyncClient, DataType as _};
use rand::Rng as _;

const CYCLE_TIME: tokio::time::Duration = tokio::time::Duration::from_millis(100);

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client = AsyncClient::new("opc.tcp://opcuademo.sterfive.com:26543", CYCLE_TIME)
        .with_context(|| "connect")?;

    // `/Root/Objects/1:Boiler#1/1:CustomController/1:Input1`
    let node_id = ua::NodeId::numeric(1, 1773);

    println!("Reading node {node_id}");

    let attributes = read_attributes(
        &client,
        &node_id,
        &[
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
        ],
    )
    .await?;

    for (attribute_id, value) in attributes {
        if let Some(value) = value.value() {
            println!("{attribute_id} -> {value:?}");
        }
    }

    let value = client.read_value(&node_id).await.with_context(|| "read")?;

    println!("-> {value:?}");

    let value = rand::thread_rng().gen_range(0.0..100.0);

    println!("Writing {value} to node {node_id}");

    client
        .write_value(
            &node_id,
            &ua::DataValue::init()
                .with_value(&ua::Variant::init().with_scalar(&ua::Double::new(value))),
        )
        .await
        .with_context(|| "write")?;

    println!("Reading node {node_id}");

    let value = client.read_value(&node_id).await.with_context(|| "read")?;

    println!("-> {value:?}");

    Ok(())
}

async fn read_attributes(
    client: &AsyncClient,
    node_id: &ua::NodeId,
    attribute_ids: &[ua::AttributeId],
) -> anyhow::Result<HashMap<ua::AttributeId, ua::DataValue>> {
    let mut result = HashMap::new();

    for attribute_id in attribute_ids {
        let value = client.read_attribute(node_id, attribute_id).await?;
        result.insert(attribute_id.clone(), value);
    }

    Ok(result)
}
