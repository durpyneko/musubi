use crate::edge::{Edge, EdgeId};
use crate::node::{Node, NodeId};

#[derive(Debug)]
pub struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Graph {
    /// Initialize a [`Graph`]
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    pub fn node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(id)
    }

    pub fn edges(&self) -> &[Edge] {
        &self.edges
    }

    /// Add a node to the [`Graph`]
    pub fn add_node(&mut self, mut node: Node) -> NodeId {
        let id = self.nodes.len();

        node.id = id;
        self.nodes.push(node);

        id
    }

    /// Connects two nodes together, creating an [`Edge`].
    pub fn connect(&mut self, from: NodeId, to: NodeId, name: String) -> EdgeId {
        let id = self.edges.len();

        self.edges.push(Edge { id, name, from, to });

        id
    }

    pub fn incoming(&self, node: NodeId) -> Vec<NodeId> {
        self.edges
            .iter()
            .filter(|edge| edge.to == node)
            .map(|edge| edge.from)
            .collect()
    }

    pub fn outgoing(&self, node: NodeId) -> Vec<NodeId> {
        self.edges
            .iter()
            .filter(|edge| edge.from == node)
            .map(|edge| edge.to)
            .collect()
    }

    // pub fn display_name(&self, node: NodeId) -> String {
    //     let node = self.node(node).unwrap();

    //     if let Some(name) = &node.name {
    //         return name.clone();
    //     }

    //     self.incoming(node.id)
    //         .iter()
    //         .filter_map(|id| self.node(*id))
    //         .filter_map(|node| node.name.as_deref())
    //         .collect::<Vec<_>>()
    //         .join(" ")
    // }

    pub fn node_display_name(&self, node: NodeId) -> String {
        let node = match self.node(node) {
            Some(node) => node,
            None => return "?".into(),
        };

        // Explicitly named nodes keep their own name.
        if let Some(name) = &node.name {
            return name.clone();
        }

        // Unnamed nodes derive their name from their inputs.
        let inputs = self.incoming(node.id);

        if inputs.is_empty() {
            return "?".into();
        }

        inputs
            .iter()
            .rev()
            .map(|input| self.node_display_name(*input))
            .filter(|name| name != "?")
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn depth(&self, node: NodeId) -> usize {
        let inputs = self.incoming(node);

        if inputs.is_empty() {
            return 0;
        }

        inputs
            .iter()
            .map(|&input| self.depth(input))
            .max()
            .unwrap_or(0)
            + 1
    }
}
