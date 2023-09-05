mod layer;
mod node;

pub use self::layer::Layer;
pub use self::node::Node;

use crate::data::KeyRange;
use rsa::{BigUint, RsaPrivateKey};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Network {
    input_len: usize,
    output_len: usize,
    pub layers: Vec<Layer>,
}

impl Network {
    pub fn new(layers_sizes: impl IntoIterator<Item = usize>) -> Network {
        let mut last_layer_size = None;
        let layers: Vec<Layer> = layers_sizes
            .into_iter()
            .map(|size| {
                let prev_size = last_layer_size.take().unwrap_or(0);
                last_layer_size = Some(size);
                Layer::new(size, prev_size)
            })
            .collect();

        Network {
            input_len: layers.first().unwrap().nodes.len(),
            output_len: layers.last().unwrap().nodes.len(),
            layers,
        }
    }

    pub fn train(
        &mut self,
        input: impl IntoIterator<Item = f32>,
        target: impl IntoIterator<Item = f32>,
        eta: f32,
    ) {
        let _output = self.calculate(input);
        self.backprop_error(target);
        self.update_weight(eta);
    }
}

impl Network {
    /// calculates the output of the network for a given input
    pub fn calculate(&mut self, input: impl IntoIterator<Item = f32>) -> Box<[f32]> {
        // TODO: Check for bad input

        self.set_input(input);

        for r in (1..self.layers.len()) {
            let [prev_layer, cur_layer] = self.layers.get_many_mut([r - 1, r]).unwrap();

            for c in (0..cur_layer.nodes.len()) {
                let node = cur_layer.nodes.get_mut(c).unwrap();

                let mut sum = node.bias;
                node.weights
                    .iter()
                    .zip(prev_layer.nodes.iter().map(|n| n.output))
                    .for_each(|(weight, output)| {
                        sum += output * weight;
                    });

                // TODO: take the sigmoid before setting
                node.output = sum
            }
        }
        let last = self.layers.last().unwrap();
        last.nodes.iter().map(|n| n.output).collect()
    }

    fn set_input(&mut self, input: impl IntoIterator<Item = f32>) {
        self.layers
            .first_mut()
            .unwrap()
            .nodes
            .iter_mut()
            .zip(input)
            .for_each(|(node, input)| {
                node.output = input;
            });
    }

    /// calculates the error, used to modify the outputDerivative that modifies the weigth(later)
    fn backprop_error(&mut self, target: impl IntoIterator<Item = f32>) {
        self.layers
            .last_mut()
            .unwrap()
            .nodes
            .iter_mut()
            .zip(target)
            .for_each(|(node, target)| {
                node.error_signal = (node.output - target) * node.output_derivative();
            });

        self.layers.iter_mut().rev().skip(1).for_each(|l| {
            l.nodes.iter_mut().for_each(|n| {
                // the sum of weights that point to it
                let mut sum = 0.;

                //             for (int nextNeuron = 0; nextNeuron < networkLayerSize[layer+1]; nextNeuron++){
                //                 sum += weight[layer+1][nextNeuron][neuron]; //called from point of veiw of next neuron
                //             }
                n.error_signal = sum * n.output_derivative();
            });
        });
    }

    /// updates the weight of each weight after each iteration
    fn update_weight(&mut self, eta: f32) {
        //     for (int layer = 1; layer < networkSize - 1; layer++){
        //         for (int neuron = 0; neuron < networkLayerSize[layer]; neuron++){
        //             double delta = -eta * errorSignal[layer][neuron];
        //             bias [layer][neuron] += delta;
        //
        //             for (int prevNeuron = 0; prevNeuron < networkLayerSize[layer - 1]; prevNeuron++){
        //                 weight[layer][neuron][prevNeuron] += output[layer-1][prevNeuron] * delta;
        //             }
        //         }
        //     }
    }
}
