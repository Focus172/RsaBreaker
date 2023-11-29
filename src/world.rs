use std::collections::HashMap;
use std::sync::{Arc, RwLock, Weak};

use rand::Rng;
use rand::distributions::Standard;
use rand::prelude::Distribution;

use crate::prelude::*;

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Uuid(usize);

impl Distribution<Uuid> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Uuid {
        Uuid(rng.gen())
    }
}
static WORLD: RwLock<HashMap<Uuid, NetworkData>> = RwLock::default();
const fn a() -> RwLock<HashMap<Uuid, NetworkData>> {
    RwLock::new(HashMap::new())

}

enum NetworkData {
    Node(Weak<Node>),
    Layer(Arc<Layer>),
}

impl NetworkData {
    fn as_node(&self) -> Option<&Weak<Node>> {
        match self {
            NetworkData::Node(n) => Some(n),
            NetworkData::Layer(_) => None,
        }
    }
}

pub fn get_node_output(uuid: &Uuid) -> Option<f32> {
    WORLD
        .read()
        .unwrap()
        .get(uuid)?
        .as_node()?
        .upgrade()
        .map(|n| n.get_output())
}

pub fn add_node(uuid: Uuid, node: Weak<Node>) {
    let data = NetworkData::Node(node);
    WORLD.write().unwrap().insert(uuid, data);
}
