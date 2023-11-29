use crate::prelude::*;
use std::{rc::Rc, sync::Arc};

use serde::{Deserialize, Serialize};

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
            crate::world::add_node(node.uuid, Arc::downgrade(node));
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
        todo!()
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

    #[deprecated = "Layer no longer needs validation. Just remove the call."]
    pub fn validate(&mut self, prev: Option<Rc<Layer>>, next: Option<Rc<Layer>>) -> Result<()> {
        unimplemented!()
        //     if self.__valid {
        //         return Err(Error::generic(
        //             "Could not validate layer beacuse is is already valid".into(),
        //         ));
        //     }
        //
        //     const CELL_ERROR: fn(Rc<Layer>) -> Error =
        //         |_| Error::generic(String::from("failed to set data in cell"));
        //
        //     if let Some(prev) = prev {
        //         self.prev.set(prev).map_err(CELL_ERROR)?;
        //     }
        //
        //     if let Some(next) = next {
        //         self.next.set(next).map_err(CELL_ERROR)?;
        //     }
        //
        //     self.__valid = true;
        //
        //     Ok(())
    }
}
