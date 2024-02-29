use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub nodes: Vec<Uuid>,
}

impl Layer {
    pub fn random<I>(size: usize, prev: I, world: &mut World) -> Self
    where
        I: IntoIterator<Item = Uuid> + Clone,
        I::IntoIter: ExactSizeIterator,
    {
        let nodes = std::iter::repeat_with(|| Node::random(prev.clone().into_iter()))
            .take(size)
            .map(|node| world.push(node))
            .collect();

        Self { nodes }
    }
}
