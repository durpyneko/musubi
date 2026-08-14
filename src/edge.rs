use crate::node::NodeId;

pub type EdgeId = usize;

#[derive(Debug)]
pub struct Edge {
    pub id: EdgeId,
    pub name: String,
    pub from: NodeId,
    pub to: NodeId,
}
