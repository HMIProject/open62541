use std::{
    sync::mpsc::{self, RecvTimeoutError},
    thread,
    time::Duration,
};

use open62541::{
    ua::{self},
    ObjectNode, Server, VariableNode,
};
use open62541_sys::{
    UA_NS0ID_BASEDATAVARIABLETYPE, UA_NS0ID_FOLDERTYPE, UA_NS0ID_OBJECTSFOLDER, UA_NS0ID_ORGANIZES,
    UA_NS0ID_STRING,
};

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let (server, runner) = Server::new();

    println!("Adding server nodes");

    let object_node = ObjectNode {
        requested_new_node_id: ua::NodeId::string(1, "the.folder"),
        parent_node_id: ua::NodeId::ns0(UA_NS0ID_OBJECTSFOLDER),
        reference_type_id: ua::NodeId::ns0(UA_NS0ID_ORGANIZES),
        browse_name: ua::QualifiedName::new(1, "the folder"),
        type_definition: ua::NodeId::ns0(UA_NS0ID_FOLDERTYPE),
        attributes: ua::ObjectAttributes::default(),
    };

    let variable_node_id = ua::NodeId::string(1, "the.answer");
    let variable_node = VariableNode {
        requested_new_node_id: variable_node_id.clone(),
        parent_node_id: object_node.requested_new_node_id.clone(),
        reference_type_id: ua::NodeId::ns0(UA_NS0ID_ORGANIZES),
        browse_name: ua::QualifiedName::new(1, "the answer"),
        type_definition: ua::NodeId::ns0(UA_NS0ID_BASEDATAVARIABLETYPE),
        attributes: ua::VariableAttributes::default()
            .with_data_type(&ua::NodeId::ns0(UA_NS0ID_STRING)),
    };

    server.add_object_node(object_node)?;
    server.add_variable_node(variable_node)?;

    server.write_variable_string(&variable_node_id, "foobar")?;

    let (cancel_tx, cancel_rx) = mpsc::channel();

    // Start background task that simulates changing variable values.
    let server_task_handle = thread::spawn(move || -> anyhow::Result<()> {
        println!("Simulating values");
        loop {
            for value in ["foo", "bar", "baz"] {
                match cancel_rx.recv_timeout(Duration::from_millis(1000)) {
                    Ok(()) => return Ok(()),
                    Err(RecvTimeoutError::Timeout) => {
                        // Continue and simulate next updated value below, then repeat loop.
                    }
                    Err(RecvTimeoutError::Disconnected) => panic!("main task should be running"),
                }
                server.write_variable_string(&variable_node_id, value)?;
            }
        }
    });

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

    cancel_tx.send(()).expect("server task should be running");

    // Wait for simulation task to shut down after canceling.
    if let Err(err) = server_task_handle
        .join()
        .expect("server task should not panic")
    {
        println!("Server task failed: {err}");
    }

    println!("Done");

    Ok(())
}
