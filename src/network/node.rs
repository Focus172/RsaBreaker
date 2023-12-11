use std::sync::{atomic::Ordering, Arc, RwLock};

use crate::prelude::*;
use atomic::AtomicF32;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(from = "InnerNode")]
pub struct Node {
    /// A floating point number that is the weight from the bias node
    pub bias: RwLock<f32>,
    /// An array of values of length equal to previous layers number of nodes that
    /// repersents how much the output of that node should be multiplied by before
    /// being added to this one.
    pub weights: RwLock<Vec<(Uuid, f32)>>,
    //pub _weights: AtomicPtr<[f32]>,
    /// repersents the output that the node emited this or last cycle
    #[serde(skip)]
    pub poutput: AtomicF32,
    /// a calculated value based on the error from last cycle
    #[serde(skip)]
    pub err_sig: AtomicF32,
    /// a unique name for the node
    #[serde(skip)]
    pub uuid: Uuid,
}

impl Node {
    pub fn random(prev: &[Uuid]) -> Arc<Self> {
        let uuid = rand::random();
        let weights = RwLock::new(
            prev.iter()
                .copied()
                .zip(util::rand::weights(prev.len(), 0.3, 0.7).into_vec())
                .collect(),
        );

        let node = Arc::new(Self {
            bias: RwLock::new(0.3),
            weights,
            uuid,
            ..Default::default()
        });

        WORLD.push(&node);

        node
    }
}

impl Node {
    #[inline]
    pub fn get_bias(&self) -> f32 {
        *self.bias.read().unwrap()
    }

    #[inline]
    pub fn get_output(&self) -> f32 {
        self.poutput.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_error_signal(&self) -> f32 {
        self.err_sig.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn output_derivative(&self) -> f32 {
        let output = self.get_output();
        output * (1. - output)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct InnerNode {
    pub bias: f32,
    pub weights: Box<[(Uuid, f32)]>,
    pub uuid: Uuid,
}

// use std::hint;

impl From<InnerNode> for Node {
    fn from(value: InnerNode) -> Self {
        Self {
            bias: RwLock::new(value.bias),
            weights: RwLock::new(value.weights.into_vec()),
            uuid: value.uuid,
            ..Default::default()
        }
    }
}
