use anyhow::{anyhow, Context as _};
use open62541_sys::{UA_NS0ID_HIERARCHICALREFERENCES, UA_NS0ID_ROOTFOLDER};
use open62541::{AsyncClient, DataType, ua};
use open62541::ua::{BrowsePath, RelativePath, RelativePathElement};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client = AsyncClient::new("opc.tcp://opcuademo.sterfive.com:26543").context("connect")?;

    let paths = vec![
        "/Root/0:Objects/2:DeviceSet/1:CoffeeMachineA/7:Parameters/17:CoffeeBeanLevel",
        "/Root/0:Objects/1:MyDevices/1:Pressure",
        "/Root/0:Objects/2:DeviceSet/1:RFIDScanner/4:ScanActive"
    ];

    // translate single browse path
    let path_with_expected_nodes = paths.iter()
        .zip(vec![
            ua::NodeId::numeric(1, 1068),
            ua::NodeId::string(1, "Pressure"),
            ua::NodeId::string(1, "RFIDScanner-ScanActive")
        ]);

    for (path, expected_node_id) in path_with_expected_nodes {
        let result_node_id = translate_browse_path(&client, path).await?;
        assert_eq!(result_node_id.node_id(), &expected_node_id);
    }


    // translate many browse paths
    translate_many_browse_path(&client, paths).await?;

    Ok(())
}

async fn translate_many_browse_path(client: &AsyncClient, paths: Vec<&str>) -> anyhow::Result<()> {
    println!("\ntranslate_many_browse_paths: ");

    let browse_paths: Vec<BrowsePath> = paths.iter().map(|p| create_browse_path(p)).collect::<Result<_, _>>()?;

    let browse_results = client.translate_many_browse_paths(&browse_paths).await?;

    for (i, browse_result) in browse_results.into_iter().enumerate() {
        println!("path: {:?} resulted in: {:?}", paths[i], browse_result);
    }
    Ok(())
}

async fn translate_browse_path(client: &AsyncClient, path: &str) -> anyhow::Result<ua::ExpandedNodeId> {

    let browse_path = create_browse_path(path)?;
    let browse_targets = client.translate_browse_path(&browse_path).await?;

    let Some(target) = browse_targets.first() else {
        Err(anyhow!("Expected one target, got {}", browse_targets.len()))?
    };

    if let Some(remaining) = target.remaining_path_index() {
        Err(anyhow!("Expected remaining path index to be None, got {}", remaining))?
    }

    let node_id = target.target_id();
    println!("translated browse path: {:?} to node_id: {:?}", path, node_id);

    Ok(node_id.clone())
}

fn create_browse_path(path: &str) -> anyhow::Result<BrowsePath> {
    let objects_path = if path.starts_with("/Root") {
        path.strip_prefix("/Root").unwrap()
    } else {
        Err(anyhow!("Invalid browse. Must start with /Root"))?
    };

    let elements = objects_path.split('/')
        .filter(|seg| !seg.is_empty())
        .filter_map(|seg| seg.split_once(':'))
        .filter_map(|(ns, name)| ns.parse::<u16>().ok().map(|ns| (ns, name)))
        .map(|(ns, name)| {
            RelativePathElement::init()
                .with_reference_type_id(&ua::NodeId::ns0(UA_NS0ID_HIERARCHICALREFERENCES))
                .with_is_inverse(false)
                .with_include_subtypes(true)
                .with_target_name(&ua::QualifiedName::new(ns, name))
        })
        .collect::<Vec<_>>();

    let r_path = RelativePath::init().with_elements(&elements);

    let browse_path = BrowsePath::init()
        .with_starting_node(&ua::NodeId::ns0(UA_NS0ID_ROOTFOLDER))
        //.with_starting_node(&ua::NodeId::ns0(UA_NS0ID_OBJECTSFOLDER))
        .with_relative_path(&r_path);

    Ok(browse_path)
}
