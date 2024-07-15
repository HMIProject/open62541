// This is heavily inspired by https://www.open62541.org/doc/master/tutorial_server_object.html

use std::{
    sync::mpsc::{self, RecvTimeoutError},
    thread,
    time::Duration,
};

use open62541::{
    ua::{self, NodeId, StatusCode},
    Node, NodeContext, NodeTypeLifecycle, Server,
};
use open62541::{Error, Lifecycle};
use open62541_sys::{
    UA_NS0ID_BASEDATAVARIABLETYPE, UA_NS0ID_BASEOBJECTTYPE, UA_NS0ID_FOLDERTYPE,
    UA_NS0ID_HASCOMPONENT, UA_NS0ID_HASMODELLINGRULE, UA_NS0ID_HASSUBTYPE,
    UA_NS0ID_MODELLINGRULE_MANDATORY, UA_NS0ID_OBJECTATTRIBUTES, UA_NS0ID_OBJECTSFOLDER,
    UA_NS0ID_OBJECTTYPEATTRIBUTES, UA_NS0ID_ORGANIZES, UA_NS0ID_STRING, UA_VALUERANK_SCALAR,
};

fn define_object_types(server: &Server, pump_type_id: &NodeId) {
    // Define the object type for "Device"
    let mut dt_attr = ua::Attributes::ObjectType(ua::ObjectTypeAttributes::default());
    dt_attr.display_name("en-US", "DeviceType");
    let mut device_type_node = Node {
        node_id: NodeId::null(),
        parent_node_id: NodeId::numeric(0, UA_NS0ID_BASEOBJECTTYPE),
        reference_type_id: NodeId::numeric(0, UA_NS0ID_HASSUBTYPE),
        browse_name: ua::QualifiedName::new(1, "DeviceType"),
        type_definition: Some(ua::NodeId::ns0(UA_NS0ID_OBJECTTYPEATTRIBUTES)),
        node_context: None,
        attributes: dt_attr,
    };
    server
        .add_node(&mut device_type_node)
        .expect("Add node failed!");

    let mut mn_attr = ua::Attributes::Variable(ua::VariableAttributes::default());
    mn_attr.display_name("en-US", "ManufacturerName");
    let mut manufacturer_name_node = Node {
        node_id: NodeId::null(),
        parent_node_id: device_type_node.node_id.clone(),
        reference_type_id: NodeId::numeric(0, UA_NS0ID_HASCOMPONENT),
        browse_name: ua::QualifiedName::new(1, "Manufacturer"),
        type_definition: Some(ua::NodeId::ns0(UA_NS0ID_BASEDATAVARIABLETYPE)),
        node_context: None,
        attributes: mn_attr,
    };
    server
        .add_node(&mut manufacturer_name_node)
        .expect("Add node failed!");

    // Make the manufacturer name mandatory
    server
        .add_reference(
            &manufacturer_name_node.node_id,
            &NodeId::numeric(0, UA_NS0ID_HASMODELLINGRULE),
            &ua::ExpandedNodeId::numeric(0, UA_NS0ID_MODELLINGRULE_MANDATORY),
            true,
        )
        .expect("Add reference failed!");

    let mut model_attr = ua::Attributes::Variable(ua::VariableAttributes::default());
    model_attr.display_name("en-US", "ModelName");
    let mut model_attr_node = Node {
        node_id: NodeId::null(),
        parent_node_id: device_type_node.node_id.clone(),
        reference_type_id: NodeId::numeric(0, UA_NS0ID_HASCOMPONENT),
        browse_name: ua::QualifiedName::new(1, "ModelName"),
        type_definition: Some(ua::NodeId::ns0(UA_NS0ID_BASEDATAVARIABLETYPE)),
        node_context: None,
        attributes: model_attr,
    };
    server
        .add_node(&mut model_attr_node)
        .expect("Add node failed!");

    // Define the object type for "Pump"
    let mut pt_attr = ua::Attributes::ObjectType(ua::ObjectTypeAttributes::default());
    pt_attr.display_name("en-US", "PumpType");
    let mut pump_type_node = Node {
        node_id: pump_type_id.clone(),
        parent_node_id: device_type_node.node_id.clone(),
        reference_type_id: NodeId::numeric(0, UA_NS0ID_HASSUBTYPE),
        browse_name: ua::QualifiedName::new(1, "PumpType"),
        type_definition: Some(ua::NodeId::ns0(UA_NS0ID_OBJECTTYPEATTRIBUTES)),
        node_context: None,
        attributes: pt_attr,
    };
    server
        .add_node(&mut pump_type_node)
        .expect("Add node failed!");

    let mut status_attr = ua::Attributes::Variable(ua::VariableAttributes::default());
    status_attr
        .display_name("en-US", "Status")
        .value_rank(UA_VALUERANK_SCALAR);
    let mut status_node = Node {
        node_id: NodeId::null(),
        parent_node_id: pump_type_id.clone(),
        reference_type_id: NodeId::numeric(0, UA_NS0ID_HASCOMPONENT),
        browse_name: ua::QualifiedName::new(1, "Status"),
        type_definition: Some(ua::NodeId::ns0(UA_NS0ID_BASEDATAVARIABLETYPE)),
        node_context: None,
        attributes: status_attr,
    };
    server.add_node(&mut status_node).expect("Add node failed!");

    // Make status variable mandatory
    server
        .add_reference(
            &status_node.node_id.clone(),
            &NodeId::numeric(0, UA_NS0ID_HASMODELLINGRULE),
            &ua::ExpandedNodeId::numeric(0, UA_NS0ID_MODELLINGRULE_MANDATORY),
            true,
        )
        .expect("Add reference failed!");

    let mut rpm_attr = ua::Attributes::Variable(ua::VariableAttributes::default());
    rpm_attr
        .display_name("en-US", "MotorRPM")
        .value_rank(UA_VALUERANK_SCALAR);
    let mut rpm_node = Node {
        node_id: NodeId::null(),
        parent_node_id: pump_type_id.clone(),
        reference_type_id: NodeId::numeric(0, UA_NS0ID_HASCOMPONENT),
        browse_name: ua::QualifiedName::new(1, "MotorRPMs"),
        type_definition: Some(ua::NodeId::ns0(UA_NS0ID_BASEDATAVARIABLETYPE)),
        node_context: None,
        attributes: rpm_attr,
    };
    server.add_node(&mut rpm_node).expect("Add node failed!");
}

fn add_pump_object_instace(
    server: &Server,
    name: &str,
    pump_type_id: &NodeId,
    node_context: NodeContext,
) {
    let mut o_attr = ua::Attributes::Object(ua::ObjectAttributes::default());
    o_attr.display_name("en-US", name);
    let mut object_node = Node {
        node_id: NodeId::null(),
        parent_node_id: NodeId::numeric(0, UA_NS0ID_OBJECTSFOLDER),
        reference_type_id: NodeId::numeric(0, UA_NS0ID_ORGANIZES),
        browse_name: ua::QualifiedName::new(1, name),
        type_definition: Some(pump_type_id.clone()),
        node_context: Some(node_context),
        attributes: o_attr,
    };
    server.add_node(&mut object_node).expect("Add node failed!");
}

struct PumpType {}

impl Lifecycle for PumpType {
    fn constructor(
        &mut self,
        server: &mut Server,
        session_id: &NodeId,
        session_context: *mut std::ffi::c_void,
        type_id: &NodeId,
        type_context: *mut std::ffi::c_void,
        node_id: &NodeId,
    ) -> ua::StatusCode {
        let qualified_name = ua::QualifiedName::new(1, "Status");
        let mut rpe = ua::RelativePathElement::new();
        rpe.init()
            .reference_type_id(&NodeId::numeric(0, UA_NS0ID_HASCOMPONENT))
            .is_inverse(false)
            .include_subtypes(false)
            .target_name(qualified_name);

        let mut bp = ua::BrowsePath::new();
        bp.init()
            .starting_node(node_id)
            .relative_path_element_size(1)
            .relative_path_elements(rpe);

        let bpr = server.translate_browse_path_to_node_ids(&bp);
        if bpr.get_status_code() != ua::StatusCode::GOOD || bpr.get_targets_size() < 1 {
            return bpr.get_status_code();
        }

        let status = ua::Boolean::new(true);
        let value = ua::Variant::scalar(status);
        server.write_variable(bpr.get_target(0).get_target_id().node_id(), &value);

        return StatusCode::GOOD;
    }
    fn destructor(
        &mut self,
        _server: &mut Server,
        _session_id: &NodeId,
        _session_context: *mut std::ffi::c_void,
        _type_id: &NodeId,
        _type_context: *mut std::ffi::c_void,
        _node_id: &NodeId,
    ) {
        todo!()
    }
}

fn add_pump_type_constructor(server: &Server, pump_type_id: &NodeId) -> NodeContext {
    let (lifecycle, node_context) = NodeTypeLifecycle::wrap_lifecycle(PumpType {});
    server.set_node_type_lifecycle(pump_type_id, lifecycle);
    node_context
}

fn main() -> anyhow::Result<()> {
    // env_logger::init();
    let env = env_logger::Env::default()
        .filter_or("RUST_LOG", "trace")
        .write_style_or("RUST_LOG", "always");
    env_logger::init_from_env(env);

    let (server, runner) = Server::new();

    println!("Adding server nodes");

    let pump_type_id = ua::NodeId::numeric(1, 1001);

    define_object_types(&server, &pump_type_id);
    let node_context = add_pump_type_constructor(&server, &pump_type_id);
    add_pump_object_instace(&server, "pump2", &pump_type_id, node_context);

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
