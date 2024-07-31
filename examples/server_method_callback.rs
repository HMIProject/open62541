use std::thread;

use open62541::{
    ua::{self, MethodAttributes},
    Attributes, DataType, MethodCallback, MethodNode, Result, Server,
};
use open62541_sys::{UA_NS0ID_HASCOMPONENT, UA_NS0ID_OBJECTSFOLDER};

struct ExampleCallback {}

impl MethodCallback for ExampleCallback {
    fn callback(
        &self,
        _session_id: ua::NodeId,
        _method_id: ua::NodeId,
        _object_id: ua::NodeId,
        _input: ua::Array<ua::Variant>,
    ) -> Result<ua::Array<ua::Variant>> {
        todo!()
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
        browse_name: ua::QualifiedName::new(1, "hello world"),
        requested_new_node_id: ua::NodeId::numeric(1, 62541),
        attributes: MethodAttributes::init()
            .with_display_name(&ua::LocalizedText::new("en-US", "Hello World")?)
            .with_executable(true)
            .with_user_executable(true),
        arguments_request_new_node_ids: None,
        input_arguments: ua::Array::from_slice(&[input_argument]),
        output_arguments: ua::Array::from_slice(&[output_argument]),
        parent_node_id: ua::NodeId::ns0(UA_NS0ID_OBJECTSFOLDER),
        reference_type_id: ua::NodeId::ns0(UA_NS0ID_HASCOMPONENT),
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
