//! Tools for parsing the network to and from the file system

use crate::prelude::*;

use super::{layer::RefLayer, Layer, Network};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ParsableNetwork {
    pub layers: Box<[Layer]>,
}

impl<'de> Deserialize<'de> for Network {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        ParsableNetwork::deserialize(deserializer).map(Into::into)
    }
}

impl Serialize for Network {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        ParsableNetwork::serialize(&self.into(), serializer)
    }
}

impl Network {
    #[inline]
    pub fn open() -> Self {
        match fs::File::open("nn.json")
            .map_err(Into::<Error>::into)
            .and_then(|f| json::from_reader::<_, Network>(f).map_err(Into::into))
        {
            Ok(n) => n,
            Err(e) => {
                log::error!("Network fetch failure: {}", e);
                log::info!("Creating bare network to remedy");
                ParsableNetwork::new_random([7, 5, 5, 3]).into()
            }
        }
    }
}

impl From<ParsableNetwork> for Network {
    fn from(value: ParsableNetwork) -> Self {
        let mut prev_layer = None;
        let layers = value
            .layers
            .into_vec()
            .into_iter()
            .map(|layer| {
                let prev = RefLayer::new(prev_layer.take(), layer.nodes);
                prev_layer = Some(prev.clone());
                prev
            })
            .collect();

        // Rc::get_mut(this)

        Self { layers }
    }
}

impl From<&Network> for ParsableNetwork {
    fn from(value: &Network) -> Self {
        let layers = value.layers.iter().map(Into::into).collect();
        Self { layers }
    }
}

impl ParsableNetwork {
    pub fn new_random<I>(sizes: I) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        let mut prev_layer_len: usize = 0;

        Self {
            layers: sizes
                .into_iter()
                .map(|size| {
                    let layer = Layer::new(size, prev_layer_len);
                    prev_layer_len = size;
                    layer
                })
                .collect(),
        }
    }
}
