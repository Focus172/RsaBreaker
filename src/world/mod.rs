use crate::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;

mod uuid;
pub use uuid::Uuid;

/// Main driver for the network. Contains all of the data in the network
#[derive(Debug, Default)]
pub struct World {
    data: HashMap<Uuid, Mutex<Node>>,
    iter: usize,
}

impl<'de> Deserialize<'de> for World {
    fn deserialize<D>(deserializer: D) -> std::prelude::v1::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let nodes = Vec::<Node>::deserialize(deserializer)?;
        let mut data = HashMap::new();

        for node in nodes {
            data.insert(node.uuid, Mutex::new(node));
        }
        Ok(Self { data, iter: 0 })
    }
}

impl Serialize for World {
    /// Consumes the world returning the underlying nodes it contains
    fn serialize<S>(&self, serializer: S) -> std::prelude::v1::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.data
            .iter()
            .map(|(_, n)| n.lock().unwrap().clone())
            .collect::<Vec<_>>()
            .serialize(serializer)
    }
}

impl World {
    /// Makes a new empty World
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a node to this world. Returns the uuid of the node.
    pub fn push(&mut self, node: Node) -> Uuid {
        let uuid = node.uuid;
        self.data.insert(uuid, Mutex::new(node));
        uuid
    }

    /// Moves to the next iteration
    pub fn next(&mut self) {
        self.iter += 1;
        log::info!("Starting iteration: {}", self.iter);
    }

    /// Beacuse you have mutable acsess to the world, you will always get the
    /// node if it exists.
    pub fn get_mut(&mut self, uuid: &Uuid) -> Option<&mut Node> {
        self.data.get_mut(uuid).and_then(|a| a.get_mut().ok())
    }

    /// Creates a proxy to this world. All non-mutable acsess should be done
    /// through this reference.
    pub fn proxy(&self) -> WorldProxy {
        WorldProxy(&self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct WorldProxy<'a>(&'a World);

impl<'a> WorldProxy<'a> {
    /// Gets the output for a given uuid if it is a node
    pub fn get_output(&self, uuid: &Uuid) -> Option<f32> {
        Node::get_output(uuid, *self)
    }

    /// Returns the current iteration of the world
    pub fn iteration(&self) -> usize {
        self.0.iter
    }

    pub fn set_input(&self, uuid: &Uuid, value: f32) {
        self.get(uuid).unwrap().set_input(value);
    }

    pub fn get(&self, uuid: &Uuid) -> Option<std::sync::MutexGuard<'_, Node>> {
        self.0.data.get(uuid).and_then(|s| s.lock().ok())
    }
}
