use open62541::{ua, ObjectNode, Server, VariableNode};
use open62541_sys::{UA_NS0ID_OBJECTSFOLDER, UA_NS0ID_ORGANIZES};

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let mut server = Server::new();

    let object_node = ObjectNode {
        requested_new_node_id: ua::NodeId::string(1, "the.folder"),
        parent_node_id: ua::NodeId::numeric(0, UA_NS0ID_OBJECTSFOLDER),
        reference_type_id: ua::NodeId::numeric(0, UA_NS0ID_ORGANIZES),
        browse_name: ua::QualifiedName::new(1, "the folder"),
        type_definition: ua::NodeId::numeric(0, 0),
        attributes: ua::ObjectAttributes::default(),
    };

    let variable_node_id = ua::NodeId::string(1, "the.answer");
    let variable_node = VariableNode {
        requested_new_node_id: variable_node_id.clone(),
        parent_node_id: object_node.requested_new_node_id.clone(),
        reference_type_id: ua::NodeId::numeric(0, UA_NS0ID_ORGANIZES),
        browse_name: ua::QualifiedName::new(1, "the answer"),
        type_definition: ua::NodeId::numeric(0, 0),
        attributes: ua::VariableAttributes::default(),
    };

    server.add_object_node(object_node)?;
    server.add_variable_node(variable_node)?;

    server.write_variable_string(variable_node_id, "foobar")?;

    server.run()?;

    Ok(())
}
