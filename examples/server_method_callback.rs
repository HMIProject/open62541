use std::thread;

use anyhow::Context as _;
use open62541::{
    ua, Attributes, DataType, MethodCallback, MethodCallbackContext, MethodCallbackError,
    MethodCallbackResult, MethodNode, Server,
};
use open62541_sys::{UA_NS0ID_HASCOMPONENT, UA_NS0ID_OBJECTSFOLDER};

struct ExampleCallback {}

impl MethodCallback for ExampleCallback {
    #[expect(clippy::get_first, reason = "show 0-based arguments in example")]
    fn call(&mut self, context: &mut MethodCallbackContext) -> MethodCallbackResult {
        let input_argument =
            context
                .input_arguments()
                .get(0)
                .ok_or(MethodCallbackError::from_status_code(
                    ua::StatusCode::BADARGUMENTSMISSING,
                ))?;

        let input_value = input_argument
            .as_scalar::<ua::String>()
            .and_then(|string| string.as_str())
            .ok_or(MethodCallbackError::from_status_code(
                ua::StatusCode::BADINVALIDARGUMENT,
            ))?;

        let output_value = ua::Variant::scalar(ua::String::new(&format!(
            "Nice input string: {input_value}"
        ))?);

        let output_argument = context
            .output_arguments_mut()
            .get_mut(0)
            .ok_or(ua::StatusCode::BADINTERNALERROR)
            .map_err(MethodCallbackError::from_status_code)?;

        *output_argument = output_value;

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let (server, runner) = Server::new();

    println!("Adding server nodes");

    let input_argument = ua::Argument::init()
        .with_data_type(&ua::NodeId::numeric(0, 12))
        .with_name(&ua::String::new("MyInput")?)
        .with_description(&ua::LocalizedText::new("en-US", "A String")?)
        .with_value_rank(-1);

    let output_argument = ua::Argument::init()
        .with_data_type(&ua::NodeId::numeric(0, 12))
        .with_name(&ua::String::new("MyOutput")?)
        .with_description(&ua::LocalizedText::new("en-US", "A String")?)
        .with_value_rank(-1);

    let method_node = MethodNode {
        requested_new_node_id: Some(ua::NodeId::numeric(1, 62541)),
        parent_node_id: ua::NodeId::ns0(UA_NS0ID_OBJECTSFOLDER),
        reference_type_id: ua::NodeId::ns0(UA_NS0ID_HASCOMPONENT),
        browse_name: ua::QualifiedName::new(1, "hello world"),
        attributes: ua::MethodAttributes::init()
            .with_display_name(&ua::LocalizedText::new("en-US", "Hello World")?)
            .with_executable(true)
            .with_user_executable(true),
        input_arguments: ua::Array::from_slice(&[input_argument]),
        input_arguments_requested_new_node_id: None,
        output_arguments: ua::Array::from_slice(&[output_argument]),
        output_arguments_requested_new_node_id: None,
    };
    let (method_node_id, _) = server.add_method_node(method_node, ExampleCallback {})?;

    // Start runner task that handles incoming connections (events).
    let runner_task_handle = thread::spawn(|| -> anyhow::Result<()> {
        println!("Running server");
        runner.run_until_interrupt()?;
        Ok(())
    });

    // Wait for runner task to finish eventually (SIGINT/Ctrl+C).
    if let Err(err) = runner_task_handle
        .join()
        .expect("runner task should not panic")
    {
        println!("Runner task failed: {err}");
    }

    println!("Exiting");

    server
        .delete_node(&method_node_id)
        .context("delete method node")?;

    println!("Done");

    Ok(())
}
