use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Driver {
    pub layers: Vec<Layer>,
}

impl Driver {
    pub fn random<I>(world: &mut World, sizes: I) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        let mut prev = Vec::new();

        let layers = sizes
            .into_iter()
            .map(|size| {
                let l = Layer::random(size, std::mem::take(&mut prev), world);
                prev = l.nodes.clone();
                l
            })
            .collect();

        Self { layers }
    }

    pub fn train<I>(&mut self, world: &mut World, input: I, target: I, eta: f32)
    where
        I: IntoIterator<Item = f32> + Clone,
    {
        self.calculate(world, input);
        self.backprop_error(world, target);
        self.update_weight(world, eta);
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
    pub fn calculate<I>(&self, world: &mut World, input: I)
    where
        I: IntoIterator<Item = f32>,
    {
        // TODO: Check for bad input

        let mut layers = self.layers.iter();

        // For each of the nodes in the first layer. Sets their input to to the
        // given values.
        if let Some(layer) = layers.next() {
            let world = world.proxy();
            // fill the remaining data with zeros
            let input = input.into_iter().chain(std::iter::repeat(0.0));

            for (uuid, value) in layer.nodes.iter().zip(input) {
                world.set_input(uuid, value)
            }
        }

        for layer in layers {
            for uuid in layer.nodes.iter() {
                // updata each of the nodes
                let _ = Node::get_output(uuid, world.proxy());
            }
        }
    }

    /// calculates the error, used to modify the outputDerivative that modifies the weigth(later)
    fn backprop_error<I>(&self, world: &mut World, target: I)
    where
        I: IntoIterator<Item = f32>,
    {
        let target = target.into_iter().chain(std::iter::repeat(0.0));
        let mut layers = self.layers.iter().rev();

        for (uuid, target) in layers.next().unwrap().nodes.iter().zip(target) {
            let output = Node::get_output(uuid, world.proxy()).unwrap();

            let node = world.get_mut(uuid).unwrap();

            let error = { (output - target) * node.output_derivative() };

            node.set_error(error);
        }

        for layer in layers {
            for uuid in layer.nodes.iter() {
                let node = world.get_mut(uuid).unwrap();
                node.update_error();
            }
        }
    }

    /// updates the weight of each weight after each iteration
    fn update_weight(&self, world: &mut World, eta: f32) {
        let world = world.proxy();

        for layer in self.layers.iter() {
            for uuid in layer.nodes.iter() {
                let mut node = world.get(uuid).unwrap();

                let delta = -eta * node.get_error_signal();

                node.bias += delta;

                for (uuid, weight) in node.weights.iter_mut() {
                    *weight += world.get_output(uuid).unwrap() * delta;
                }
            }
        }
    }

    /// returns the output of this network
    pub fn output(&self, world: WorldProxy) -> Box<[f32]> {
        self.layers
            .last()
            .unwrap()
            .nodes
            .iter()
            .map(|uuid| world.get_output(uuid).unwrap())
            .collect()
    }
}
