use super::Node;

pub struct Layer {
    nodes: Vec<Node>,
}

impl Layer {
    pub fn new(number_nodes: usize) -> Layer {
        Layer {
            nodes: (0..number_nodes).map(|_| Node::new()).collect(),
        }
    }
}
