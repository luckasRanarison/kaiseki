#[derive(Debug, Default, Clone)]
pub struct Node {
    pub start: usize,
    pub end: usize,
    pub context_id: i16,
    pub cost: u32,
    pub total_cost: i32,
}

pub struct Lattice {
    nodes: Vec<Node>,
    starts_at: Vec<Vec<usize>>,
    ends_at: Vec<Vec<usize>>,
}

impl Lattice {
    pub fn new(len: usize) -> Self {
        let nodes = vec![Node::default(); 2];
        let mut starts_at = vec![vec![]; len];
        let mut ends_at = vec![vec![]; len + 1];

        ends_at[0].push(0);
        starts_at[len].push(1);

        Self {
            nodes,
            starts_at,
            ends_at,
        }
    }

    pub fn add_node(&mut self, node: Node) {
        let node_id = self.nodes.len();
        self.starts_at[node.start].push(node_id);
        self.ends_at[node.end].push(node_id);
        self.nodes.push(node);
    }
}
