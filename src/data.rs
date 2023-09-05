use core::task;
use std::ops::Range;

// use tokio::sync::mpsc::TryRecvError;

use tokio::sync::mpsc;
use tokio::task::JoinHandle;

use rand::rngs::ThreadRng;

use rsa::{traits::PrivateKeyParts, BigUint, RsaPrivateKey, RsaPublicKey};

use crate::traits::AsTrainingData;

// fn print(&self) {
//    println!("Public keys: {} {}", self.pub_key.0, self.pub_key.1);
//    println!("Private keys: {} {}", self.priv_key.0, self.priv_key.1);
// }

/// An infinite, async iterable data source for key pairs
pub struct Data {
    rx: mpsc::Receiver<(RsaPublicKey, RsaPrivateKey)>,
    handle: Option<JoinHandle<()>>,
}

impl Data {
    const BITS: usize = 64;
    // const BITS: usize = 2048;

    pub async fn new() -> Data {
        let (tx, rx) = mpsc::channel(16);

        let handle = tokio::spawn(async move {
            while !tx.is_closed() {
                // the thread rng is droped at the end of this block so this fn is send
                let priv_key = {
                    let mut rng = rand::thread_rng();
                    RsaPrivateKey::new(&mut rng, Self::BITS).expect("failed to generate a key")
                };

                let publ_key = RsaPublicKey::from(&priv_key);
                tx.send((publ_key, priv_key)).await;
            }
        });

        Data {
            rx,
            handle: Some(handle),
        }
    }

    async fn close(&mut self) {
        self.rx.close();
        let h = self
            .handle
            .take()
            .expect("You cannont close the data channel twice");
        h.await;
    }

    pub async fn next(&mut self) -> Option<(RsaPublicKey, RsaPrivateKey)> {
        self.rx.recv().await
    }
}

impl Drop for Data {
    fn drop(&mut self) {
        self.close();
    }
}

pub struct KeyRange {
    bounds: Vec<Range<BigUint>>,
}

impl KeyRange {
    pub fn contains(&self, priv_key: RsaPrivateKey) -> bool {
        // TODO: check for size differences in the lists

        priv_key
            .primes()
            .iter()
            .zip(self.bounds.iter())
            .all(|(factor, range)| range.contains(factor))
    }
}
