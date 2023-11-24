mod layer;
mod node;
mod parse;

use crate::prelude::*;
use std::rc::Rc;

pub use self::layer::Layer;
use self::layer::RefLayer;
pub use self::node::Node;

#[derive(Debug, Clone)]
pub struct Network {
    pub layers: Rc<[RefLayer]>,
}

impl Network {
    pub fn train<I>(&self, input: I, target: I, eta: f32)
    where
        I: IntoIterator<Item = f32>,
    {
        self.calculate(input);
        self.backprop_error(target);
        self.update_weight(eta);
    }

    // input_len: usize,
    // output_len: usize,

    /// calculates the output of the network for a given input
    pub fn calculate<I>(&self, input: I)
    where
        I: IntoIterator<Item = f32>,
    {
        // TODO: Check for bad input

        self.set_input(input);

        for curr_layer in self.layers.iter() {
            let Some(prev_layer) = curr_layer.prev.clone() else {
                continue;
            };

            for node in curr_layer.nodes.iter() {
                let mut sum = node.get().bias;
                node.get()
                    .weights
                    .iter()
                    .zip(prev_layer.nodes.iter().map(|n| n.output()))
                    .for_each(|(weight, output)| {
                        sum += output * weight;
                    });

                node.set_output(sum);
            }
        }
    }

    fn set_input<I>(&self, input: I)
    where
        I: IntoIterator<Item = f32>,
    {
        // fill the remaining data with zeros
        let input = input.into_iter().chain(std::iter::repeat(0.0));

        self.layers
            .first()
            .unwrap()
            .nodes
            .iter()
            .zip(input)
            .for_each(|(node, input)| {
                node.set_output(input);
            });
    }

    /// calculates the error, used to modify the outputDerivative that modifies the weigth(later)
    fn backprop_error<I>(&self, target: I)
    where
        I: IntoIterator<Item = f32>,
    {
        let target = target.into_iter().chain(std::iter::repeat(0.0));
        self.layers
            .last()
            .unwrap()
            .nodes
            .iter()
            .zip(target)
            .for_each(|(node, target)| {
                node.set_error_signal((node.output() - target) * node.output_derivative())
            });

        for next_layer in self.layers.iter().rev() {
            let Some(curr_layer) = next_layer.prev.clone() else {
                continue;
            };

            for (index, node) in curr_layer.nodes.iter().enumerate() {
                // the sum of weights that point to it
                let importance = next_layer
                    .nodes
                    .iter()
                    .map(|n| n.get().weights[index])
                    .sum::<f32>();

                node.set_error_signal(importance * node.output_derivative());
            }
        }
    }

    /// updates the weight of each weight after each iteration
    fn update_weight(&self, eta: f32) {
        for curr_layer in self.layers.iter() {
            let Some(prev_layer) = curr_layer.prev.clone() else {
                continue;
            };

            for node in curr_layer.nodes.iter() {
                let delta = -eta * node.error_signal();
                node.get_mut().bias += delta;

                prev_layer
                    .nodes
                    .iter()
                    .map(|n| n.output())
                    .zip(node.get_mut().weights.iter_mut())
                    .for_each(|(output, weight)| {
                        *weight += output * delta;
                    });
            }
        }
    }

    /// returns the output of this network
    pub fn output(&self) -> Box<[f32]> {
        self.layers
            .last()
            .unwrap()
            .nodes
            .iter()
            .map(|n| n.output())
            .collect()
    }
}

fn sigmoid(i: f32) -> f32 {
    1. / (1. + f32::exp(-i))
}
