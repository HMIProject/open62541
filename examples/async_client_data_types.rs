use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    mem,
    str::FromStr as _,
};

use anyhow::Context as _;
use itertools::Itertools;
use open62541::{AsyncClient, ClientBuilder, ua};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    for fetch_upfront in [true, false] {
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

    // let node_id = ua::NodeId::from_str("ns=3;s=\"6601_HMI\".\"Freshwater\"").unwrap();
    let node_id = ua::NodeId::from_str("ns=3;s=\"6601_HMI\".\"Filling\"").unwrap();

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

        let data_type_descriptions =
            read_nested_data_type_descriptions(&client, &[data_type_id], &[]).await?;

        println!("Data type descriptions: {data_type_descriptions:?}");

        // let data_type = description.to_data_type(None).context("create data type")?;
        // println!("Data type: {data_type:?}");

        // println!("Encoded value: {extension_object_value:?}");
        // let member =
        //     data_type.get_struct_member(&mut extension_object_value, "active_flushing2")?;
        // println!("Member value: {member:?}");
        // println!("Decoded value: {extension_object_value:?}");
    }

    println!("Disconnecting client");

    client.disconnect().await;

    Ok(())
}

async fn read_nested_data_type_descriptions(
    client: &AsyncClient,
    data_type_ids: &[ua::NodeId],
    known_data_type_ids: &[ua::NodeId],
) -> anyhow::Result<Vec<ua::DataTypeDescription>> {
    enum KnownDataType {
        PriorKnowledge,
        FoundNow(ua::DataTypeDescription),
    }

    const MAX_READ_DATA_TYPE_DESCRIPTION_LEN: usize = 100;

    let mut known_data_types =
        BTreeMap::from_iter(known_data_type_ids.iter().map(|known_data_type_id| {
            (known_data_type_id.to_owned(), KnownDataType::PriorKnowledge)
        }));
    let mut pending_data_type_ids = BTreeSet::from_iter(data_type_ids.iter().cloned());

    while !pending_data_type_ids.is_empty() {
        let data_type_ids = pending_data_type_ids
            .iter()
            .take(MAX_READ_DATA_TYPE_DESCRIPTION_LEN)
            .collect::<Vec<_>>();

        let data_type_descriptions = read_data_type_descriptions(client, &data_type_ids).await?;

        for data_type_id in find_nested_data_type_ids(&data_type_descriptions)? {
            if !known_data_type_ids.contains(&data_type_id) && data_type_id.as_ns0().is_none() {
                pending_data_type_ids.insert(data_type_id);
            }
        }

        for data_type_description in data_type_descriptions {
            let data_type_id = data_type_description.data_type_id();

            pending_data_type_ids.remove(data_type_id);
            known_data_types.insert(
                data_type_id.to_owned(),
                KnownDataType::FoundNow(data_type_description),
            );
        }
    }

    Ok(known_data_types
        .into_values()
        .filter_map(|known_data_type| match known_data_type {
            KnownDataType::PriorKnowledge => None,
            KnownDataType::FoundNow(data_type_description) => Some(data_type_description),
        })
        .collect())
}

async fn read_data_type_descriptions(
    client: &AsyncClient,
    data_type_ids: &[&ua::NodeId],
) -> anyhow::Result<Vec<ua::DataTypeDescription>> {
    let node_attributes = data_type_ids
        .iter()
        .flat_map(|&data_type_id| {
            [
                // Match attributes with processing below.
                (data_type_id.to_owned(), ua::AttributeId::BROWSENAME),
                (data_type_id.to_owned(), ua::AttributeId::DATATYPEDEFINITION),
            ]
        })
        .collect::<Vec<_>>();

    println!(
        "Reading {} data type definitions: {:?}",
        data_type_ids.len(),
        data_type_ids
    );
    let attribute_values = client.read_many_attributes(&node_attributes).await?;

    let mut data_type_descriptions = Vec::with_capacity(data_type_ids.len());

    // Match attributes with read request above.
    for (&data_type_id, (browse_name, data_type_definition)) in data_type_ids
        .iter()
        .zip(attribute_values.into_iter().tuples())
    {
        let browse_name = browse_name
            .into_scalar_value()
            .context("require browse name value")?
            .into_scalar::<ua::QualifiedName>()
            .context("unexpected browse name type")?;

        let data_type_definition = ua::DataTypeDefinition::from_abstract(
            data_type_definition
                .into_scalar_value()
                .context("require data type definition value")?
                .into_scalar::<ua::Variant>()
                .context("unexpected data type definition type")?,
        )?;

        let structure_definition = match data_type_definition {
            ua::DataTypeDefinition::Structure(definition) => definition,
            ua::DataTypeDefinition::Enum(_) => {
                anyhow::bail!("unsupported enum definition")
            }
            _ => anyhow::bail!("unsupported data type definition"),
        };

        let structure_description =
            structure_definition.into_description(data_type_id.to_owned(), browse_name);

        data_type_descriptions.push(structure_description.into_abstract());
    }

    Ok(data_type_descriptions)
}

fn find_nested_data_type_ids(
    data_type_descriptions: &[ua::DataTypeDescription],
) -> anyhow::Result<Vec<ua::NodeId>> {
    let mut nested_data_type_ids = Vec::new();

    for data_type_description in data_type_descriptions {
        match data_type_description.to_definition() {
            ua::DataTypeDefinition::Structure(definition) => {
                for field in definition.fields().context("missing struct fields")?.iter() {
                    nested_data_type_ids.push(field.data_type().to_owned());
                }
            }
            ua::DataTypeDefinition::Enum(_) => {
                anyhow::bail!("unsupported enum definition")
            }
            _ => anyhow::bail!("unsupported data type definition"),
        }
    }

    Ok(nested_data_type_ids)
}
