use serde::{Serialize, Deserialize};
use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Node {
    /// A floating point number that 
    pub bias: f32,
    /// An array of values of length equal to previous layers number of nodes that
    /// repersents how much the output of that node should be multiplied by before 
    /// being added to this one
    pub weights: Vec<f32>,
    #[serde(skip)]
    /// repersents the output that the node emited this or last cycle
    pub output: f32,
    #[serde(skip)]
    pub error_signal: f32,
}

impl Node {
    pub fn new(perv_layer_size: usize) -> Self {
        Node {
            bias: 0.3,
            weights: util::rand::weights(perv_layer_size, 0.3, 0.7),
            output: 0.,
            error_signal: 0.,
        }
    }

    pub fn output_derivative(&self) -> f32 {
        self.output * (1. - self.output)
    }
}
