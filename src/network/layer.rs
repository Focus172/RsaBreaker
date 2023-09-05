use serde::{Deserialize, Serialize};

use super::Node;

#[derive(Serialize, Deserialize, Default)]
pub struct Layer {
    pub nodes: Vec<Node>,
}

impl Layer {
    pub fn new(number_nodes: usize, prev_layer_size: usize) -> Layer {
        Layer {
            nodes: (0..number_nodes)
                .map(|_| Node::new(prev_layer_size))
                .collect(),
        }
    }

}
