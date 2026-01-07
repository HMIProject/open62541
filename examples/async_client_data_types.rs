use std::str::FromStr as _;

use anyhow::Context as _;
use open62541::{ClientBuilder, ua};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    for fetch_upfront in [false, true] {
        read_value(fetch_upfront).await?;
    }

    println!("Exiting");

    Ok(())
}

async fn read_value(fetch_upfront: bool) -> anyhow::Result<()> {
    let mut client = ClientBuilder::default()
        .connect("opc.tcp://10.92.40.124:4841")
        .context("connect")?;
    if fetch_upfront {
        client = client
            .with_remote_data_types()
            .context("get remote data types")?
    }
    let client = client.into_async();

    println!("Connected successfully (fetch_upfront: {fetch_upfront})");

    let node_id = ua::NodeId::from_str("ns=3;s=\"6601_HMI\".\"Freshwater\"").unwrap();

    let value = client
        .read_value(&node_id)
        .await
        .context("read value")?
        .into_value()
        .context("turn into value")?;
    let type_id = value.type_id().context("get type ID")?;

    println!("Raw value: {value:?}");
    println!("Type ID: {type_id:?}");

    if let Some(mut extension_object_value) = value.to_scalar::<ua::ExtensionObject>() {
        let data_type_id = client
            .read_attribute(&node_id, ua::AttributeId::DATATYPE_T)
            .await
            .context("read data type")?
            .into_scalar_value()
            .context("turn into scalar value")?;
        let name = client
            .read_attribute(&node_id, ua::AttributeId::BROWSENAME_T)
            .await
            .context("read data type")?
            .into_scalar_value()
            .context("turn into scalar value")?;

        println!("Data type ID: {data_type_id:?}");
        println!("Data type name: {name:?}");

        let data_type_definition = client
            .read_attribute(&data_type_id, ua::AttributeId::DATATYPEDEFINITION_T)
            .await
            .context("read data type")?
            .into_scalar_value()
            .context("turn into scalar value")?;
        println!("Data type definition: {data_type_definition:?}");

        let ua::DataTypeDefinition::Structure(structure_definition) = data_type_definition else {
            anyhow::bail!("require structure definition");
        };

        let base_data_type = structure_definition.base_data_type();
        println!("Base data type: {base_data_type:?}");
        let description = structure_definition.into_description(name);
        println!("Data type description: {description:?}");

        let data_type = ua::DataType::from_description(ua::ExtensionObject::new(&description))
            .context("create data type")?;

        println!("Encoded value: {extension_object_value:?}");
        let member =
            data_type.get_struct_member(&mut extension_object_value, "active_flushing2")?;
        println!("Member value: {member:?}");
        println!("Decoded value: {extension_object_value:?}");
    }

    println!("Disconnecting client");

    client.disconnect().await;

    Ok(())
}
