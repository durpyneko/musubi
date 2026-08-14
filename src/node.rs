pub type NodeId = usize;

#[derive(Debug)]
pub struct Node {
    pub id: NodeId,
    pub name: Option<String>,
}

impl Node {
    pub fn new(name: Option<String>) -> Self {
        Self { id: 0, name }
    }

    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("?")
    }
}
