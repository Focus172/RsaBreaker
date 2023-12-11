use std::collections::HashMap;
use std::sync::{self, Arc, LazyLock, Mutex, Weak};

use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::Rng;

use crate::prelude::*;

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// ---
#[derive(Serialize, Deserialize)]
pub struct Uuid(usize);

impl Distribution<Uuid> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Uuid {
        Uuid(rng.gen())
    }
}

pub static WORLD: World = World::new();

pub struct World {
    inner: LazyLock<Mutex<HashMap<Uuid, NetworkData>>>,
}

impl World {
    pub const fn new() -> Self {
        Self {
            inner: LazyLock::new(|| Mutex::new(HashMap::new())),
        }
    }

    pub fn push(&self, node: &Arc<Node>) {
        let uuid = node.uuid;
        let data = NetworkData::Node(Arc::downgrade(node));
        self.inner.lock().unwrap().insert(uuid, data);
    }

    pub fn push_layer(&self, uuid: Uuid, layer: Arc<Layer>) {
        let data = NetworkData::Layer(layer);
        self.inner.lock().unwrap().insert(uuid, data);
    }

    pub fn get_output(&self, uuid: &Uuid) -> Option<f32> {
        self.inner
            .lock()
            .unwrap()
            .get(uuid)
            .and_then(NetworkData::as_node)
            .and_then(sync::Weak::upgrade)
            .map(|n| n.get_output())
    }
}

enum NetworkData {
    Node(sync::Weak<Node>),
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
