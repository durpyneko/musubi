use crate::graph::Graph;
use crate::node::NodeId;

#[derive(Debug)]
pub struct PositionedNode {
    pub node: NodeId,
    pub x: usize,
    pub y: usize,
}

pub fn layout(graph: &Graph) -> Vec<PositionedNode> {
    let mut positions = Vec::new();

    let mut root_y = 0;

    // Position root nodes first.
    for node in graph.nodes() {
        if graph.incoming(node.id).is_empty() {
            positions.push(PositionedNode {
                node: node.id,
                x: graph.depth(node.id),
                y: root_y,
            });

            root_y += 2;
        }
    }

    // Position everything that depends on the roots.
    for node in graph.nodes() {
        if graph.incoming(node.id).is_empty() {
            continue;
        }

        calculate_position(graph, node.id, &mut positions);
    }

    positions
}

fn calculate_position(graph: &Graph, node_id: NodeId, positions: &mut Vec<PositionedNode>) {
    // Don't calculate the same node twice.
    if positions.iter().any(|position| position.node == node_id) {
        return;
    }

    let inputs = graph.incoming(node_id);

    // Make sure all inputs have positions first.
    for input in &inputs {
        calculate_position(graph, *input, positions);
    }

    // Get the positions of our inputs.
    let input_positions: Vec<&PositionedNode> = inputs
        .iter()
        .filter_map(|input| positions.iter().find(|position| position.node == *input))
        .collect();

    if input_positions.is_empty() {
        return;
    }

    // Place this node between its inputs.
    let y = input_positions
        .iter()
        .map(|position| position.y)
        .sum::<usize>()
        / input_positions.len();

    positions.push(PositionedNode {
        node: node_id,
        x: graph.depth(node_id),
        y,
    });
}
