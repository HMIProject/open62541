use std::{
    collections::{hash_map::Entry, HashMap, VecDeque},
    mem,
};

use anyhow::Context as _;
use open62541::{ua, AsyncClient, DataType as _, Result};
use open62541_sys::UA_NS0ID_SERVERTYPE;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client = AsyncClient::new("opc.tcp://opcuademo.sterfive.com:26543").context("connect")?;

    let hierarchy = browse_hierarchy(&client, &ua::NodeId::numeric(0, UA_NS0ID_SERVERTYPE)).await?;

    hierarchy.pretty_print(|name, (node_id, node)| {
        let Some(name) = name else {
            return format!("{node_id}");
        };

        let info = format!("{}, {:?}", node.display_name().text(), node.node_class());

        format!("{name} ({info}) -> {node_id}")
    });

    Ok(())
}

/// Maximum number of node IDs to browse in single request.
const MAX_NODE_IDS: usize = 1000;

async fn browse_hierarchy(
    client: &AsyncClient,
    root_node_id: &ua::NodeId,
) -> anyhow::Result<TreeNode<(ua::NodeId, ua::ReferenceDescription)>> {
    // Maps parent node IDs to list of children (node IDs with their respective names).
    let mut children: HashMap<ua::NodeId, Vec<ua::ReferenceDescription>> = HashMap::new();

    let mut pending_node_ids = VecDeque::new();
    pending_node_ids.push_back(root_node_id.clone());

    while !pending_node_ids.is_empty() {
        let browse_node_ids: Vec<ua::NodeId> = pending_node_ids
            .drain(..MAX_NODE_IDS.min(pending_node_ids.len()))
            .collect();

        println!(
            "Browsing {} node IDs {browse_node_ids:?}",
            browse_node_ids.len()
        );

        let results = browse_many_contd(client, &browse_node_ids)
            .await
            .context("browse")?;

        for (node_id, references) in browse_node_ids.into_iter().zip(results) {
            let references = match references {
                Err(err) => {
                    println!("Ignoring node `{node_id}` that cannot be browsed: {err}");
                    continue;
                }
                Ok(references) => references,
            };

            let children = match children.entry(node_id) {
                Entry::Occupied(child) => {
                    let node_id = child.key();
                    println!("Ignoring child `{node_id}` reachable on different paths");
                    continue;
                }
                Entry::Vacant(entry) => entry.insert(Vec::new()),
            };

            for reference in references {
                pending_node_ids.push_back(reference.node_id().node_id().clone());
                children.push(reference);
            }
        }
    }

    let root_node = (root_node_id.clone(), ua::ReferenceDescription::init());

    let tree_node = TreeNode::build_tree(root_node, move |(node_id, _)| {
        children
            .remove(node_id)
            .unwrap_or_else(|| {
                println!("Assuming no children for unbrowsed child `{node_id}`");
                Vec::new()
            })
            .into_iter()
            .map(|browse_description| {
                (
                    browse_description.browse_name().to_string(),
                    (
                        browse_description.node_id().node_id().clone(),
                        browse_description,
                    ),
                )
            })
    });

    Ok(tree_node)
}

/// Exhaustively browses several nodes at once.
///
/// This consumes any continuation points that might be returned from browsing, ensuring that all
/// references are returned eventually.
async fn browse_many_contd(
    client: &AsyncClient,
    node_ids: &[ua::NodeId],
) -> Result<Vec<Result<Vec<ua::ReferenceDescription>>>> {
    let mut results = client.browse_many(node_ids).await?;

    debug_assert_eq!(results.len(), node_ids.len());
    // Tracks index of the original node ID for this result index.
    let mut result_indices: Vec<usize> = (0..results.len()).collect();
    // Collects all references for the given original node ID (index).
    let mut collected_references: Vec<Result<Vec<ua::ReferenceDescription>>> =
        (0..results.len()).map(|_| Ok(Vec::new())).collect();

    loop {
        let mut continuation_points = Vec::new();

        // Walk results from previous iteration. Use associated index to know which node ID was
        // browsed (or continued to be browsed). While consuming the old `result_indices`, build a
        // list of continuation points with matching new `result_indices` for every browse that is
        // still not complete.
        for (index, result) in mem::take(&mut result_indices).into_iter().zip(results) {
            match result {
                Ok((references, continuation_point)) => {
                    // PANIC: Once browsing has failed for a node, we never continue browsing the
                    // node again. (In fact, we could not if we wanted as we have no continuation
                    // point).
                    collected_references[index]
                        .as_mut()
                        .expect("should not have failed when browsing continues")
                        .extend(references);

                    if let Some(continuation_point) = continuation_point {
                        continuation_points.push(continuation_point);
                        result_indices.push(index);
                    }
                }

                Err(err) => {
                    // When browsing fails, take note of error. But when continuation of browsing
                    // fails, this also throws away any previously accumulated references. That is
                    // okay for now, because most errors should manifest in the first browse call.
                    collected_references[index] = Err(err);
                }
            }
        }

        // When every browse has returned without continuation point, we are done.
        if continuation_points.is_empty() {
            break;
        }

        // Otherwise, use continuation points to continue browsing (only) those node IDs that have
        // still references to be returned from the server. Use `result_indices` to remember which
        // index from `continuation_points` belongs to which index in `collected_references`.
        results = client.browse_next(&continuation_points).await?;
        debug_assert_eq!(results.len(), result_indices.len());
    }

    Ok(collected_references)
}

#[derive(Debug)]
struct TreeNode<T> {
    value: T,
    children: HashMap<String, TreeNode<T>>,
}

impl<T> TreeNode<T> {
    /// Builds tree from root using child list callback.
    fn build_tree<FC, C>(root_value: T, mut children: FC) -> Self
    where
        FC: FnMut(&T) -> C,
        C: Iterator<Item = (String, T)>,
    {
        let mut root_node = TreeNode {
            value: root_value,
            children: HashMap::new(),
        };

        let mut pending_nodes = vec![&mut root_node];

        while let Some(node) = pending_nodes.pop() {
            debug_assert!(node.children.is_empty());

            for (child_name, child_value) in children(&node.value) {
                let child_node = TreeNode {
                    value: child_value,
                    children: HashMap::new(),
                };

                match node.children.entry(child_name) {
                    Entry::Occupied(entry) => {
                        println!("Ignoring duplicate child with name `{}`", entry.key());
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(child_node);
                    }
                }
            }

            pending_nodes.extend(node.children.values_mut());
        }

        root_node
    }

    fn pretty_print<FL>(&self, mut label: FL)
    where
        FL: FnMut(Option<&str>, &T) -> String,
    {
        struct PrintNode<'a, T> {
            name: Option<&'a str>,
            node: &'a TreeNode<T>,
            level: usize,
        }

        let mut pending_nodes = VecDeque::new();
        pending_nodes.push_back(PrintNode {
            name: None,
            node: self,
            level: 0,
        });

        while let Some(node) = pending_nodes.pop_back() {
            let has_children = !node.node.children.is_empty();
            let prefix = "  ".repeat(node.level) + (if has_children { "+" } else { "-" }) + " ";

            let label = label(node.name, &node.node.value);

            println!("{prefix}{label}");

            let mut children: Vec<(&String, &TreeNode<T>)> = node.node.children.iter().collect();

            children.sort_by_key(|&(child_name, _)| child_name);

            for (child_name, child_node) in children.into_iter().rev() {
                pending_nodes.push_back(PrintNode {
                    name: Some(child_name),
                    node: child_node,
                    level: node.level + 1,
                });
            }
        }
    }
}
