use std::{
    collections::{BTreeMap, BTreeSet},
    str::FromStr as _,
};

use anyhow::Context as _;
use itertools::Itertools;
use open62541::{
    AsyncClient, ClientBuilder, DataType,
    ua::{self, DataTypeArray},
};
use open62541_sys::{
    UA_NS0ID_BASEDATATYPE, UA_NS0ID_HASSUBTYPE, UA_NS0ID_PUBLISHSUBSCRIBETYPE_ADDCONNECTION,
    UA_NS0ID_REFERENCELISTENTRYDATATYPE, UA_NS0ID_VARIABLETYPENODE_ENCODING_DEFAULTJSON,
};

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
    let mut client = client.into_async();

    println!("Connected successfully (fetch_upfront: {fetch_upfront})");

    // let node_id = ua::NodeId::from_str("ns=3;s=\"6601_HMI\".\"Freshwater\"").unwrap();
    // let node_id = ua::NodeId::from_str("ns=3;s=\"6601_HMI\".\"Filling\"").unwrap();
    let node_id = ua::NodeId::from_str("ns=3;s=\"6601_HMI\".\"M901\"").unwrap();

    let value = client
        .read_value(&node_id)
        .await
        .context("read value")?
        .into_value()
        .context("turn into value")?;
    println!("Raw value: {value:?}");

    let type_id = value.type_id().context("get type ID")?;
    println!("Type ID: {type_id:?}");

    if let Some(mut extension_object_value) = value.to_scalar::<ua::ExtensionObject>() {
        let data_type_id = client
            .read_attribute(&node_id, ua::AttributeId::DATATYPE_T)
            .await
            .context("read data type")?
            .into_scalar_value()
            .context("turn into scalar value")?;

        let mut data_type_descriptions =
            read_nested_data_type_descriptions(&client, &[data_type_id], &[]).await?;
        println!("Data type descriptions: {data_type_descriptions:?}");

        let data_type_ids = data_type_descriptions
            .iter()
            .map(|data_type_description| data_type_description.data_type_id().to_owned())
            .collect::<BTreeSet<_>>();

        let mut missing_types = BTreeSet::new();

        for data_type_description in &mut data_type_descriptions {
            let ua::DataTypeDescription::Structure(structure_description) = data_type_description
            else {
                unimplemented!();
            };

            for field in structure_description
                .structure_definition()
                .fields()
                .unwrap()
                .iter_mut()
            {
                let data_type_id = field.data_type();
                if !data_type_id.is_ns0() && !data_type_ids.contains(data_type_id) {
                    println!("MISSING: {data_type_id:?}");
                    missing_types.insert(data_type_id.to_owned());
                }
            }
        }

        if !missing_types.is_empty() {
            let supertypes = read_supertypes(&client, &missing_types).await?;
            println!("Supertypes: {supertypes:?}");
            for (subtype, supertype) in missing_types.into_iter().zip(supertypes) {
                for data_type_description in data_type_descriptions.iter_mut().skip(1) {
                    data_type_description.replace_data_type(&subtype, &supertype);
                    // data_type_description
                    //     .replace_data_type(&subtype, &ua::NodeId::ns0(UA_NS0ID_BASEDATATYPE));
                }
            }
            for data_type_description in &mut data_type_descriptions {
                data_type_description.drop_arrays();
            }
            println!("Adjusted data type descriptions: {data_type_descriptions:?}");
        }

        let number_of_new_data_types = client.add_data_types(&data_type_descriptions)?;
        println!("Added {number_of_new_data_types} new data types");

        println!("Encoded value: {extension_object_value:?}");
        client.decode_extension_object(&mut extension_object_value)?;
        println!("Decoded value: {extension_object_value:?}");
        // let member =
        //     data_type.get_struct_member(&mut extension_object_value, "active_flushing2")?;
        // println!("Member value: {member:?}");
        // println!("Decoded value: {extension_object_value:?}");
    }

    let value = client
        .read_value(&node_id)
        .await
        .context("read value")?
        .into_value()
        .context("turn into value")?;
    println!("Raw value: {value:?}");

    let type_id = value.type_id().context("get type ID")?;
    println!("Type ID: {type_id:?}");

    println!("Disconnecting client");

    client.disconnect().await;

    Ok(())
}

async fn read_supertypes(
    client: &AsyncClient,
    data_type_ids: impl IntoIterator<Item = &ua::NodeId>,
) -> anyhow::Result<Vec<ua::NodeId>> {
    let browse_descriptions = data_type_ids
        .into_iter()
        .map(|data_type_id| {
            ua::BrowseDescription::init()
                .with_node_id(data_type_id)
                .with_browse_direction(&ua::BrowseDirection::INVERSE)
                .with_reference_type_id(&ua::NodeId::ns0(UA_NS0ID_HASSUBTYPE))
                .with_include_subtypes(true)
                .with_node_class_mask(&ua::NodeClassMask::DATATYPE)
                .with_result_mask(&ua::BrowseResultMask::NONE)
        })
        .collect::<Vec<_>>();

    let results = client.browse_many(&browse_descriptions).await?;

    let mut supertypes = Vec::with_capacity(results.len());

    for result in results {
        let result = result?;

        if !result.1.is_none() {
            unimplemented!();
        };

        let nodes = result.0.to_vec();
        assert_eq!(nodes.len(), 1);

        supertypes.push(nodes.first().unwrap().node_id().node_id().to_owned());
    }

    Ok(supertypes)
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
    let mut pending_data_type_ids = BTreeSet::from_iter(
        data_type_ids
            .iter()
            .filter(|data_type_id| !is_well_known_data_type(data_type_id))
            .cloned(),
    );

    while !pending_data_type_ids.is_empty() {
        let data_type_ids = pending_data_type_ids
            .iter()
            .take(MAX_READ_DATA_TYPE_DESCRIPTION_LEN)
            .collect::<Vec<_>>();

        let data_type_descriptions = read_data_type_descriptions(client, &data_type_ids).await?;

        for data_type_id in find_nested_data_type_ids(&data_type_descriptions)? {
            if !known_data_type_ids.contains(&data_type_id)
                && !is_well_known_data_type(&data_type_id)
            {
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

        data_type_descriptions.push(ua::DataTypeDescription::Structure(structure_description));
    }

    Ok(data_type_descriptions)
}

fn find_nested_data_type_ids(
    data_type_descriptions: &[ua::DataTypeDescription],
) -> anyhow::Result<BTreeSet<ua::NodeId>> {
    let mut nested_data_type_ids = BTreeSet::new();

    for data_type_description in data_type_descriptions {
        match data_type_description {
            ua::DataTypeDescription::Structure(description) => {
                let definition = description.structure_definition();
                let fields = definition.fields().context("missing struct fields")?;
                for field in fields.iter() {
                    nested_data_type_ids.insert(field.data_type().to_owned());
                }
            }
            ua::DataTypeDescription::Enum(_) => {
                anyhow::bail!("unsupported enum description")
            }
            _ => anyhow::bail!("unsupported data type description"),
        }
    }

    Ok(nested_data_type_ids)
}

fn is_well_known_data_type(data_type_id: &ua::NodeId) -> bool {
    // TODO: Add proper support for Simatic data types.
    data_type_id.is_ns0() || (data_type_id.namespace_index() == 3 && data_type_id.is_numeric())
}
