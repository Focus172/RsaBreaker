use crate::data::KeySet;

use super::Layer;
use super::KeyRange;

pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn new(layers_sizes: impl IntoIterator<Item = usize>) -> Network {
        Network {
            layers: layers_sizes
                .into_iter()
                .map(|size| Layer::new(size))
                .collect(),
        }
    }

    pub fn train(&mut self, keys: KeySet, eta: f32) {}

    pub fn make_guess(&self, pub_key: rsa::RsaPublicKey) -> KeyRange {
        todo!()
    }
}

