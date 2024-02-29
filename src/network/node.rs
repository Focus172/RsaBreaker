use crate::prelude::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Node {
    /// A floating point number that is the weight from the bias node
    pub bias: f32,
    /// An array of values of length equal to previous layers number of nodes that
    /// repersents how much the output of that node should be multiplied by before
    /// being added to this one.
    pub weights: Vec<(Uuid, f32)>,
    /// a unique name for the node
    pub uuid: Uuid,
    /// The iteration that this node was last updated on
    #[serde(skip)]
    pub iteration: usize,

    /// repersents the output that the node emited this or last cycle
    #[serde(skip)]
    poutput: f32,
    /// a calculated value based on the error from last cycle
    #[serde(skip)]
    err_sig: f32,
}

impl Node {
    pub fn random<I>(prev: I) -> Self
    where
        I: Iterator<Item = Uuid> + ExactSizeIterator,
    {
        let uuid = rand::random();
        let len = prev.len();
        let weights = prev
            .zip(util::rand::weights(len, 0.3, 0.7).into_vec())
            .collect();

        Self {
            bias: 0.3,
            weights,
            uuid,
            ..Default::default()
        }
    }
}

impl Node {
    /// Sets this output of this node to the given value. This is only allowed
    /// when the node is an input node.
    ///
    /// # Errors
    /// If the given node has any nodes that it depends on an error will be
    /// returned.
    pub fn set_input(&mut self, value: f32) {
        debug_assert!(self.weights.is_empty());
        self.poutput = value;
    }

    #[inline]
    pub fn get_error_signal(&self) -> f32 {
        self.err_sig
    }

    pub fn update_error(&mut self) {
        // the sum of weights that point to it
        let importance: f32 = self.weights.iter().map(|(_, w)| w).sum();

        self.err_sig = importance * self.output_derivative();
    }

    pub fn set_error(&mut self, value: f32) {
        self.err_sig = value;
    }

    #[inline]
    pub fn output_derivative(&self) -> f32 {
        // HACK: this is with the assumption that it is always called after get_output
        let output = self.poutput;
        output * (1. - output)
    }

    pub fn get_output(uuid: &Uuid, world: WorldProxy) -> Option<f32> {
        let node = world.get(uuid)?;
        if node.iteration == world.iteration() {
            Some(node.poutput)
        } else {
            let mut n = world.get(uuid)?;
            let v = n
                .weights
                .iter()
                // .zip(curr_layer.prev.iter().map(get_node_output))
                .map(|(uuid, weight)| world.get_output(uuid).unwrap() * weight)
                .sum::<f32>()
                + n.bias;

            let v = sigmoid(v);

            n.iteration += 1;
            n.poutput = v;
            Some(v)
        }
    }
}
