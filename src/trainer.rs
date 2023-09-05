use std::marker::PhantomData;

use rsa::{RsaPublicKey, RsaPrivateKey};

use crate::prelude::*;

use crate::{network::Network, traits::AsTrainingData};

// #[derive(Debug)]
pub struct Trainer {
    pub nn: Network,
}

impl Trainer {
    pub fn new(nn: Network) -> Self {
        Self {
            nn,
        }
    }

    pub fn train(&mut self, input: RsaPublicKey, target: RsaPrivateKey) {
        let data = input.as_training_data();
        let target = target.as_training_data();
        self.nn.train(data.iter().cloned(), target.iter().cloned(), 0.1)
    }

    pub fn into_inner(self) -> Network {
        self.nn
    }
}
