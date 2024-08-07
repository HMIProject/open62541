use std::thread;

use open62541::{
    ua, Attributes, DataType, MethodCallback, MethodCallbackContext, MethodCallbackResult,
    MethodNode, Server,
};
use open62541_sys::{UA_NS0ID_HASCOMPONENT, UA_NS0ID_OBJECTSFOLDER};

struct ExampleCallback {}

impl MethodCallback for ExampleCallback {
    fn call(&mut self, context: &mut MethodCallbackContext) -> MethodCallbackResult {
        let input = context.input_arguments();
        let input_string = input[0].to_scalar::<ua::String>().unwrap();
        let input_string = input_string.as_str().unwrap();
        let output_string1: String = "Nice input string: ".to_owned();
        let output_string2 = output_string1 + input_string;
        let output_string3: ua::String = ua::String::new(&output_string2).unwrap();
        let output_variant = [ua::Variant::scalar(output_string3)];
        let output_array: ua::Array<ua::Variant> = ua::Array::from_slice(&output_variant);

        let binding = output_array[0].to_scalar::<ua::String>().unwrap();
        let _string1 = binding.as_str().unwrap();
        context
            .set_output_arguments(output_array.as_slice())
            .unwrap();

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let (server, runner) = Server::new();

    println!("Adding server nodes");

    let input_argument: ua::Argument = ua::Argument::init()
        .with_data_type(ua::NodeId::numeric(0, 12))
        .with_name(ua::String::new("MyInput")?)
        .with_description(ua::LocalizedText::new("en-US", "A String")?)
        .with_value_rank(-1);

    let output_argument: ua::Argument = ua::Argument::init()
        .with_data_type(ua::NodeId::numeric(0, 12))
        .with_name(ua::String::new("MyOutput")?)
        .with_description(ua::LocalizedText::new("en-US", "A String")?)
        .with_value_rank(-1);

    let method_node = MethodNode {
        requested_new_node_id: ua::NodeId::numeric(1, 62541),
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

    server.add_method_node(method_node, ExampleCallback {})?;

    // Start runner task that handles incoming connections (events).
    let runner_task_handle = thread::spawn(|| -> anyhow::Result<()> {
        println!("Running server");
        runner.run()?;
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

    println!("Done");

    Ok(())
}
