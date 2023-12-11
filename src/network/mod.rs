mod layer;
mod node;

use std::sync::atomic::Ordering;
use std::sync::Arc;

use crate::prelude::*;

pub use self::layer::Layer;
pub use self::node::Node;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Network {
    pub layers: Vec<Arc<Layer>>,
}

impl Network {
    pub fn open() -> Self {
        match fs::File::open("nn.json")
            .map_err(Into::<Error>::into)
            .and_then(|f| json::from_reader::<_, Network>(f).map_err(Into::into))
        {
            Ok(net) => {
                // Rc::get_mut(net.layers.first_mut().unwrap())
                //     .unwrap()
                //     .validate(None);
                //
                // let mut prev = net.layers.first().cloned();
                //
                // for layer in net.layers.iter_mut().skip(1) {
                //     Rc::get_mut(layer).unwrap().validate(prev.take());
                //     prev = Some(layer.clone())
                // }
                net
            }
            Err(e) => {
                log::error!("Network fetch failure: {}", e);
                log::info!("Creating bare network to remedy");
                Network::random([7, 5, 5, 3])
            }
        }
    }

    pub fn random<I>(sizes: I) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        let mut prev: Vec<_> = vec![];

        let layers = sizes
            .into_iter()
            .map(|size| {
                let l = Layer::random(size, prev.drain(..));
                prev.extend(l.nodes.iter().map(|node| node.uuid));
                Arc::new(l)
            })
            .collect();

        Self { layers }
    }

    pub fn train<I>(&self, input: I, target: I, eta: f32)
    where
        I: IntoIterator<Item = f32> + Clone,
    {
        self.calculate(input);
        self.backprop_error(target);
        self.update_weight(eta);
    }

    pub fn add_layer(&mut self, size: usize) {
        todo!()
    }

    #[inline]
    pub fn input_len(&self) -> usize {
        self.layers.first().unwrap().nodes.len()
    }

    #[inline]
    pub fn output_len(&self) -> usize {
        self.layers.last().unwrap().nodes.len()
    }

    /// calculates the output of the network for a given input
    pub fn calculate<I>(&self, input: I)
    where
        I: IntoIterator<Item = f32>,
    {
        // TODO: Check for bad input

        self.set_input(input);

        for curr_layer in self.layers.iter() {
            for node in curr_layer.nodes.iter() {
                let sum = node.get_bias()
                    + node
                        .weights
                        .read()
                        .unwrap()
                        .iter()
                        // .zip(curr_layer.prev.iter().map(get_node_output))
                        .map(|(uuid, weight)| WORLD.get_output(uuid).unwrap() * weight)
                        .sum::<f32>();

                node.poutput.store(sigmoid(sum), Ordering::Relaxed);
            }
        }
    }

    fn set_input<I>(&self, input: I)
    where
        I: IntoIterator<Item = f32>,
    {
        // fill the remaining data with zeros
        let input = input.into_iter().chain(std::iter::repeat(0.0));

        for (node, input) in self.layers.first().unwrap().nodes.iter().zip(input) {
            node.poutput.store(input, Ordering::Relaxed)
        }
    }

    /// calculates the error, used to modify the outputDerivative that modifies the weigth(later)
    fn backprop_error<I>(&self, target: I)
    where
        I: IntoIterator<Item = f32>,
    {
        let target = target.into_iter().chain(std::iter::repeat(0.0));
        for (node, target) in self.layers.last().unwrap().nodes.iter().zip(target) {
            let error = { (node.get_output() - target) * node.output_derivative() };

            node.err_sig.store(error, Ordering::Relaxed)
        }

        for [curr_layer, next_layer] in self.layers.array_windows().rev() {
            for (index, node) in curr_layer.nodes.iter().enumerate() {
                // the sum of weights that point to it
                let importance = next_layer
                    .nodes
                    .iter()
                    .map(|n| n.weights.read().unwrap()[index].1)
                    .sum::<f32>();

                node.err_sig
                    .store(importance * node.output_derivative(), Ordering::Relaxed);
            }
        }
    }

    /// updates the weight of each weight after each iteration
    fn update_weight(&self, eta: f32) {
        for layer in self.layers.iter() {
            for node in layer.nodes.iter() {
                let delta = -eta * node.get_error_signal();
                // node.bias.fetch_add(delta, Ordering::Relaxed);

                node.weights
                    .write()
                    .unwrap()
                    .iter_mut()
                    .for_each(|(uuid, weight)| {
                        *weight += WORLD.get_output(uuid).unwrap() * delta;
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
            .map(|n| n.get_output())
            .collect()
    }
}

fn sigmoid(i: f32) -> f32 {
    1. / (1. + f32::exp(-i))
}
