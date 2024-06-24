use std::thread;

use anyhow::Ok;
use open62541::{
    ua::{self, DataSource},
    DataSourceVariableNode, ObjectNode, Server,
};
use open62541_sys::{
    UA_Boolean, UA_DataValue, UA_NodeId, UA_NumericRange, UA_Server, UA_StatusCode,
    UA_Variant_setScalarCopy, UA_NS0ID_BASEDATAVARIABLETYPE, UA_NS0ID_FLOAT, UA_NS0ID_FOLDERTYPE,
    UA_NS0ID_OBJECTSFOLDER, UA_NS0ID_ORGANIZES, UA_STATUSCODE_BADNODATA, UA_STATUSCODE_GOOD,
    UA_TYPES, UA_TYPES_FLOAT,
};

static mut CPU_TEMPERATURE: f32 = 38.5;
const INCREMENT: f32 = 0.1;

fn read_cpu_temperature() -> anyhow::Result<f32> {
    let cpu_temperature: f32;
    let mut increasing = true;

    unsafe {
        println!("Current CPU Temperature: {CPU_TEMPERATURE:.2} °C");
        if CPU_TEMPERATURE >= 48.5 {
            increasing = false;
        } else if CPU_TEMPERATURE <= 38.5 {
            increasing = true;
        }
        if increasing {
            CPU_TEMPERATURE += INCREMENT;
        } else {
            CPU_TEMPERATURE -= INCREMENT;
        }
        cpu_temperature = CPU_TEMPERATURE;
    }
    Ok(cpu_temperature)
}

extern "C" fn read_cpu_temperature_callback(
    _server: *mut UA_Server,
    _session_id: *const UA_NodeId,
    _session_context: *mut ::core::ffi::c_void,
    _node_id: *const UA_NodeId,
    _node_context: *mut ::core::ffi::c_void,
    _include_source_time_stamp: UA_Boolean,
    _range: *const UA_NumericRange,
    value: *mut UA_DataValue,
) -> UA_StatusCode {
    match read_cpu_temperature() {
        core::result::Result::Ok(cpu_temperature) => {
            let cpu_temp_ptr: *const f32 = &cpu_temperature;
            let void_ptr: *const ::std::ffi::c_void = cpu_temp_ptr.cast::<::std::ffi::c_void>();
            let index = usize::try_from(UA_TYPES_FLOAT).unwrap();
            if index > unsafe { UA_TYPES.len() } {
                return UA_STATUSCODE_BADNODATA;
            }
            unsafe {
                UA_Variant_setScalarCopy(&mut (*value).value, void_ptr, &UA_TYPES[index]);
                (*value).set_hasValue(true);
            }
        }
        Err(_) => return UA_STATUSCODE_BADNODATA,
    }
    UA_STATUSCODE_GOOD
}

const extern "C" fn write_cpu_temperature_callback(
    _server: *mut UA_Server,
    _session_id: *const UA_NodeId,
    _session_context: *mut ::core::ffi::c_void,
    _node_id: *const UA_NodeId,
    _node_context: *mut ::core::ffi::c_void,
    _range: *const UA_NumericRange,
    _value: *const UA_DataValue,
) -> UA_StatusCode {
    UA_STATUSCODE_GOOD
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let (server, runner) = Server::new();

    println!("Adding server nodes");

    let object_node = ObjectNode {
        requested_new_node_id: ua::NodeId::string(1, "Controller"),
        parent_node_id: ua::NodeId::ns0(UA_NS0ID_OBJECTSFOLDER),
        reference_type_id: ua::NodeId::ns0(UA_NS0ID_ORGANIZES),
        browse_name: ua::QualifiedName::new(1, "the folder"),
        type_definition: ua::NodeId::ns0(UA_NS0ID_FOLDERTYPE),
        attributes: ua::ObjectAttributes::default(),
    };

    let cpu_temperature_data_source = DataSource::new(
        Some(read_cpu_temperature_callback),
        Some(write_cpu_temperature_callback),
    );

    let cpu_temperature_data_source_variable = ua::NodeId::string(1, "cpu_temperature");
    let data_source_variable_node = DataSourceVariableNode {
        requested_new_node_id: cpu_temperature_data_source_variable.clone(),
        parent_node_id: object_node.requested_new_node_id.clone(),
        reference_type_id: ua::NodeId::ns0(UA_NS0ID_ORGANIZES),
        browse_name: ua::QualifiedName::new(1, "cpu temperature"),
        type_definition: ua::NodeId::ns0(UA_NS0ID_BASEDATAVARIABLETYPE),
        attributes: ua::VariableAttributes::default()
            .with_data_type(&ua::NodeId::ns0(UA_NS0ID_FLOAT)),
        data_source: cpu_temperature_data_source,
    };

    server.add_object_node(object_node)?;
    server.add_data_source_variable_node(data_source_variable_node)?;

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
