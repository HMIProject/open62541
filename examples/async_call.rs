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
) -> anyhow::Result<Vec<ua::Variant>> {
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
    // Look at children of the method node. We expect properties for input and output arguments.
    // TODO: Allow limiting set of returned children by passing filters to `BrowseDescription`.
    let (references, _) = client.browse(method_node_id).await?;

    // Either of input and output arguments may be absent when the method has no arguments.
    let mut input_arguments = None;
    let mut output_arguments = None;

    for reference in &references {
        // This skips over all non-property children.
        // TODO: Use filter in `browse()` to remove other children upfront.
        match property_name(reference) {
            Some(INPUT_ARGUMENTS_PROPERTY_NAME) => {
                input_arguments = Some(reference.node_id().node_id().clone());
            }
            Some(OUTPUT_ARGUMENTS_PROPERTY_NAME) => {
                output_arguments = Some(reference.node_id().node_id().clone());
            }
            _ => {}
        }
    }

    // Use a single request to read input and output arguments in one go (if found).
    let node_values = read_sparse_node_values(client, &[input_arguments, output_arguments]).await?;

    let [ref input_arguments, ref output_arguments] = node_values[..] else {
        // PANIC: We give two node IDs to get two values.
        panic!("should have expected number of values");
    };

    let input_arguments = input_arguments.as_ref().map(get_arguments).transpose()?;
    let output_arguments = output_arguments.as_ref().map(get_arguments).transpose()?;

    Ok(MethodDefinition {
        input_arguments,
        output_arguments,
    })
}

/// Reads values from sparse list of nodes.
///
/// Returns the same number of results as the given list. Bubbles errors to the top-level `Result`.
/// Positions with `None` in `node_ids` lead to corresponding `None` entries in the resulting list.
async fn read_sparse_node_values(
    client: &AsyncClient,
    node_ids: &[Option<ua::NodeId>],
) -> anyhow::Result<Vec<Option<ua::DataValue>>> {
    // Condense sparse list into dense list for request.
    let node_attributes: Vec<_> = node_ids
        .iter()
        .flatten()
        .map(|node_id| (node_id.clone(), ua::AttributeId::VALUE))
        .collect();

    // Empty requests would return `BadNothingToDo` error.
    let values = if node_attributes.is_empty() {
        Vec::new()
    } else {
        client.read_many_attributes(&node_attributes).await?
    };

    let mut result = Vec::with_capacity(node_ids.len());
    let mut values = values.into_iter();

    for node_id in node_ids {
        match node_id {
            Some(_) => result.push(Some(values.next().expect("should have value")?)),
            None => result.push(None),
        }
    }

    debug_assert!(values.next().is_none());

    Ok(result)
}

/// Extracts argument definitions from property.
///
/// This looks into the value returned from reading `InputArguments` and `OutputArguments` property
/// and returns the list of argument names and their value types.
fn get_arguments(value: &ua::DataValue) -> anyhow::Result<Vec<(ua::String, ValueType)>> {
    // `InputArguments` and `OutputArguments` nodes are expected to hold an array of objects of the
    // `Argument` type.

    let arguments = value
        .value()
        .ok_or(anyhow::anyhow!("should have value"))?
        .to_array::<ua::Argument>()
        .ok_or(anyhow::anyhow!("should have array"))?;

    Ok(arguments
        .iter()
        .map(|argument| (argument.name().clone(), argument.value_type()))
        .collect())
}

/// Gets name of referenced property.
///
/// Returns `None` for non-property references.
fn property_name(reference: &ua::ReferenceDescription) -> Option<&str> {
    // TODO: Add methods for these checks?
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
