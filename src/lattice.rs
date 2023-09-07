use crate::matrix::CostMatrix;

const START_ID: usize = 0;
const END_ID: usize = 1;

type NodeId = usize;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Node {
    pub start: usize,
    pub end: usize,
    pub context_id: u16,
    pub cost: i16,
    pub total_cost: i32,
    pub prev_node: Option<NodeId>,
}

impl Node {
    pub fn new(start: usize, end: usize, context_id: u16, cost: i16) -> Self {
        Self {
            start,
            end,
            context_id,
            cost,
            total_cost: i32::MAX,
            prev_node: None,
        }
    }
}

pub struct Lattice {
    nodes: Vec<Node>,
    starts_at: Vec<Vec<NodeId>>,
    ends_at: Vec<Vec<NodeId>>,
}

impl Lattice {
    pub fn new(len: usize) -> Self {
        let start_node = Node::default();
        let end_node = Node::new(len, len, 0, i16::MAX);
        let nodes = vec![start_node, end_node];
        let mut starts_at = vec![vec![]; len + 1];
        let mut ends_at = vec![vec![]; len + 1];

        ends_at[0].push(START_ID);
        starts_at[len].push(END_ID);

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

    pub fn find_path(&mut self, matrix: &CostMatrix) -> Vec<Node> {
        let len = self.starts_at.len();

        for i in 0..len {
            let left_edges = &self.ends_at[i];
            let right_edges = &self.starts_at[i];

            for &current_id in right_edges {
                for &prev_id in left_edges {
                    let prev_node = self.get_node(prev_id);
                    let current = self.get_node(current_id);
                    let prev_cost = prev_node.total_cost;
                    let current_cost = current.cost;
                    let connection_cost = matrix.get(prev_node.context_id, prev_node.context_id);
                    let total_cost = prev_cost + current_cost as i32 + connection_cost as i32;

                    if total_cost < current.total_cost {
                        let mut node = &mut self.nodes[current_id];
                        node.total_cost = total_cost;
                        node.prev_node = Some(prev_id);
                    }
                }
            }
        }

        self.build_path()
    }

    pub fn has_node_ending_at(&self, index: usize) -> bool {
        !self.ends_at[index].is_empty()
    }

    fn build_path(&self) -> Vec<Node> {
        let mut nodes = Vec::new();
        let mut prev_node = self.get_node(END_ID).prev_node;

        while let Some(prev_id) = prev_node {
            let node = self.get_node(prev_id);
            nodes.push(node.clone());
            prev_node = node.prev_node;
        }

        nodes.pop();
        nodes.reverse();

        nodes
    }

    fn get_node(&self, id: NodeId) -> &Node {
        &self.nodes[id]
    }
}

#[cfg(test)]
mod tests {
    use super::{Lattice, Node};
    use crate::matrix::CostMatrix;

    #[test]
    fn test_find_path() {
        let mut lattice = Lattice::new(9);
        let cost_matrix = CostMatrix::default().unwrap();

        lattice.add_node(Node::new(0, 3, 5, 6245));
        lattice.add_node(Node::new(3, 6, 3, 10791));
        lattice.add_node(Node::new(6, 9, 5, 7595));
        lattice.add_node(Node::new(6, 9, 6, 9428));
        lattice.add_node(Node::new(3, 9, 3, 2135));
        lattice.add_node(Node::new(0, 6, 3, 3003));

        let nodes = lattice.find_path(&cost_matrix);

        assert_eq!(2, nodes.len());
    }
}
