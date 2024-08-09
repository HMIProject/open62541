use std::thread;

use anyhow::Context as _;
use open62541::{
    ua, DataSource, DataSourceError, DataSourceReadContext, DataSourceResult,
    DataSourceWriteContext, ObjectNode, Server, VariableNode,
};
use open62541_sys::{
    UA_NS0ID_BASEDATAVARIABLETYPE, UA_NS0ID_FOLDERTYPE, UA_NS0ID_OBJECTSFOLDER, UA_NS0ID_ORGANIZES,
    UA_NS0ID_STRING,
};

struct DynamicDataSource {
    current_value: String,
}

impl DynamicDataSource {
    fn new(initial_value: impl Into<String>) -> Self {
        Self {
            current_value: initial_value.into(),
        }
    }
}

impl DataSource for DynamicDataSource {
    fn read(&mut self, context: &mut DataSourceReadContext) -> DataSourceResult {
        println!("Reading data source value");
        let value = ua::Variant::scalar(
            // We do not expect strings with NUL bytes.
            ua::String::new(&self.current_value)
                .map_err(|_| DataSourceError::from_status_code(ua::StatusCode::BADINTERNALERROR))?,
        );
        println!("-> {value:?}");
        context.set_variant(value);
        Ok(())
    }

    fn write(&mut self, context: &mut DataSourceWriteContext) -> DataSourceResult {
        println!("Writing data source value");
        let value = context.value();
        println!("<- {value:?}");
        let value = value
            .value()
            // We require that the write request holds a value.
            .ok_or(DataSourceError::from_status_code(
                ua::StatusCode::BADINVALIDARGUMENT,
            ))?
            .as_scalar::<ua::String>()
            // The incoming value to write must be a string.
            .ok_or(DataSourceError::from_status_code(
                ua::StatusCode::BADINVALIDARGUMENT,
            ))?
            .as_str()
            // The incoming string must be valid UTF-8.
            .ok_or(DataSourceError::from_status_code(
                ua::StatusCode::BADINVALIDARGUMENT,
            ))?
            .into();
        self.current_value = value;
        Ok(())
    }
}

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
            .with_data_type(&ua::NodeId::ns0(UA_NS0ID_STRING))
            .with_access_level(
                &ua::AccessLevel::NONE
                    .with_current_read(true)
                    .with_current_write(true),
            ),
    };

    let data_source = DynamicDataSource::new("Lorem ipsum");

    server
        .add_object_node(object_node)
        .context("add object node")?;
    server
        .add_data_source_variable_node(variable_node, data_source)
        .context("add variable node")?;

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

    server
        .delete_node(&variable_node_id)
        .context("delete variable node")?;

    println!("Done");

    Ok(())
}
