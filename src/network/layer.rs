use std::rc::Rc;

use serde::{Deserialize, Serialize};

use super::{node::RefNode, Node};

#[derive(Debug, Clone)]
pub struct RefLayer {
    pub prev: Option<Box<RefLayer>>,
    pub nodes: Rc<[RefNode]>,
}

impl RefLayer {
    pub fn new(prev: Option<RefLayer>, nodes: Box<[Node]>) -> Self {
        let prev = prev.map(Box::new);
        Self {
            prev,
            nodes: nodes.into_vec().into_iter().map(Into::into).collect(),
        }
    }
}

impl From<&RefLayer> for Layer {
    fn from(value: &RefLayer) -> Self {
        let nodes = value.nodes.iter().map(Into::into).collect();
        Layer { nodes }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub nodes: Box<[Node]>,
}

impl Layer {
    pub fn new(size: usize, prev_size: usize) -> Layer {
        Layer {
            nodes: std::iter::repeat_with(|| Node::new(prev_size))
                .take(size)
                .collect(),
        }
    }
}
