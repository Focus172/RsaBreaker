use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::Node;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "InnerLayer")]
#[serde(into = "InnerLayer")]
pub struct Layer {
    // pub prev: Option<Arc<Layer>>,
    // pub next: Option<Arc<Layer>>,
    pub nodes: Vec<Arc<Node>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InnerLayer {
    nodes: Vec<Arc<Node>>,
    // prev: Vec<Uuid>,
    // next: Vec<Uuid>,
}

impl From<InnerLayer> for Layer {
    fn from(value: InnerLayer) -> Self {
        for node in value.nodes.iter() {
            WORLD.push(node);
        }
        Self {
            // prev: value.prev,
            // next: value.next,
            nodes: value.nodes,
        }
    }
}

impl From<Layer> for InnerLayer {
    fn from(value: Layer) -> Self {
        Self { nodes: value.nodes }
    }
}

impl Layer {
    pub fn random<I>(size: usize, prev: I) -> Self
    where
        I: IntoIterator<Item = Uuid>,
    {
        let prev: Vec<_> = prev.into_iter().collect();
        Self {
            nodes: std::iter::repeat_with(|| Node::random(&prev))
                .take(size)
                .collect(),
        }
    }
    // pub fn new(prev: Option<Rc<Layer>>, nodes: Box<[Node]>) -> Self {
    //     Self {
    //         prev,
    //         nodes: nodes.into_vec().into_iter().map(Into::into).collect(),
    //     }
    // }
    pub fn add_node(&mut self) {
        todo!()

        // let size = self.prev.get().map(|l| l.nodes.len()).unwrap_or(0);

        // let new = Rc::new(Node::random(size));

        // self.nodes.push(new);
    }
}
