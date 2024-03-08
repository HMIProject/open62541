use anyhow::{anyhow, Context as _};
use open62541::{ua, AsyncClient, DataType as _, ValueType};
use open62541_sys::{UA_NS0ID_HASPROPERTY, UA_NS0ID_PROPERTYTYPE};

const CYCLE_TIME: tokio::time::Duration = tokio::time::Duration::from_millis(100);

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client = AsyncClient::new("opc.tcp://opcuademo.sterfive.com:26543", CYCLE_TIME)
        .context("connect")?;

    // `/Root/Objects/10:Simulation/10:ObjectWithMethods`
    let object_node_id = ua::NodeId::string(10, "ObjectWithMethods");
    // `/Root/Objects/10:Simulation/10:ObjectWithMethods/10:MethodNoArgs`
    let method_no_args_node_id = ua::NodeId::string(10, "MethodNoArgs");
    // `/Root/Objects/10:Simulation/10:ObjectWithMethods/10:MethodIO`
    let method_io_node_id = ua::NodeId::string(10, "MethodIO");

    let _output_arguments =
        call_method(&client, &object_node_id, &method_no_args_node_id, &[]).await?;

    println!();

    let output_arguments = call_method(
        &client,
        &object_node_id,
        &method_io_node_id,
        &[ua::Variant::init().with_scalar(&ua::UInt32::new(123))],
    )
    .await?;

    let value: i32 = output_arguments
        .ok_or(anyhow!("output arguments"))?
        .first()
        .ok_or(anyhow!("output argument"))?
        .to_scalar::<ua::Int32>()
        .ok_or(anyhow!("scalar"))?
        .value();

    println!("-> {value}");

    Ok(())
}

async fn call_method(
    client: &AsyncClient,
    object_node_id: &ua::NodeId,
    method_node_id: &ua::NodeId,
    input_arguments: &[ua::Variant],
) -> anyhow::Result<Option<Vec<ua::Variant>>> {
    println!("Getting method definition of node {method_node_id}");

    let definition = get_definition(client, method_node_id).await?;

    println!(
        "- input arguments: {:?}",
        definition.input_arguments.unwrap_or_default()
    );
    println!(
        "- output arguments: {:?}",
        definition.output_arguments.unwrap_or_default()
    );

    println!("Calling node {method_node_id}");

    let output_arguments = client
        .call_method(object_node_id, method_node_id, input_arguments)
        .await
        .context("call")?;

    println!("-> {output_arguments:?}");

    Ok(output_arguments)
}

#[derive(Debug)]
struct MethodDefinition {
    input_arguments: Option<Vec<(ua::String, ValueType)>>,
    output_arguments: Option<Vec<(ua::String, ValueType)>>,
}

const INPUT_ARGUMENTS_PROPERTY_NAME: &str = "InputArguments";
const OUTPUT_ARGUMENTS_PROPERTY_NAME: &str = "OutputArguments";

async fn get_definition(
    client: &AsyncClient,
    method_node_id: &ua::NodeId,
) -> anyhow::Result<MethodDefinition> {
    let (references, _) = client.browse(method_node_id).await?;

    let mut input_arguments = None;
    let mut output_arguments = None;

    for reference in &references {
        match property_name(reference) {
            Some(INPUT_ARGUMENTS_PROPERTY_NAME) => input_arguments = Some(reference.node_id()),
            Some(OUTPUT_ARGUMENTS_PROPERTY_NAME) => output_arguments = Some(reference.node_id()),
            _ => {}
        }
    }

    let x = [input_arguments, output_arguments]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    // TODO: Refactor. Query input/output argument definitions in single request.
    let input_arguments = match input_arguments {
        Some(arguments) => Some(get_arguments(client, arguments.node_id()).await?),
        None => None,
    };
    let output_arguments = match output_arguments {
        Some(arguments) => Some(get_arguments(client, arguments.node_id()).await?),
        None => None,
    };

    Ok(MethodDefinition {
        input_arguments,
        output_arguments,
    })
}

async fn read_nodes(
    client: &AsyncClient,
    node_ids: &[Option<ua::NodeId>],
) -> anyhow::Result<Vec<Option<ua::DataValue>>> {
    let values: Vec<_> = node_ids
        .iter()
        .flatten()
        .map(|node_id| (node_id.clone(), ua::AttributeId::VALUE))
        .collect();

    let values = client.read_many_attributes(&values).await?;

    let _: Vec<_> = node_ids
        .iter()
        .enumerate()
        .flat_map(|(index, node_id)| node_id.as_ref().map(|_| index))
        .zip(values)
        .collect();

    todo!()
}

async fn get_arguments(
    client: &AsyncClient,
    arguments: &ua::NodeId,
) -> anyhow::Result<Vec<(ua::String, ValueType)>> {
    let arguments = client.read_value(arguments).await?;

    let arguments = arguments
        .value()
        .ok_or(anyhow::anyhow!("should have value"))?
        .to_array::<ua::ExtensionObject>()
        .ok_or(anyhow::anyhow!("should have array"))?;

    arguments
        .iter()
        .map(|object| {
            let argument = object
                .decoded_content::<ua::Argument>()
                .ok_or(anyhow::anyhow!("should have argument"))?;
            Ok((argument.name().clone(), argument.value_type()))
        })
        .collect()
}

/// Gets name of referenced property.
fn property_name(reference: &ua::ReferenceDescription) -> Option<&str> {
    (reference.node_class() == &ua::NodeClass::VARIABLE
        && reference.reference_type_id().as_ns0() == Some(UA_NS0ID_HASPROPERTY)
        && reference.type_definition().node_id().as_ns0() == Some(UA_NS0ID_PROPERTYTYPE))
    .then(|| {
        reference
            .browse_name()
            .as_ns0()
            .and_then(|name| name.as_str())
    })
    .flatten()
}
