use std::ops::Range;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

use rsa::{traits::PrivateKeyParts, BigUint, RsaPrivateKey, RsaPublicKey};

// use std::sync::Barrier;

/// An infinite, async iterable data source for key pairs
pub struct Data {
    rx: mpsc::Receiver<(RsaPublicKey, RsaPrivateKey)>,
    handle: Option<JoinHandle<()>>,
    sent: usize,
}

impl Data {
    const BITS: usize = 64;
    // const BITS: usize = 2048;

    pub fn new() -> Data {
        let (tx, rx) = mpsc::channel(16);

        let handle = tokio::spawn(async move {
            while !tx.is_closed() {
                // the thread rng is droped at the end of this block so this fn is send
                let priv_key = {
                    let mut rng = rand::thread_rng();
                    RsaPrivateKey::new(&mut rng, Self::BITS).expect("failed to generate a key")
                };

                let publ_key = RsaPublicKey::from(&priv_key);
                match tx.send((publ_key, priv_key)).await {
                    Ok(_) => {}
                    Err(_) => break,
                }
            }
        });

        Data {
            rx,
            handle: Some(handle),
            sent: 0,
        }
    }

    pub async fn close(&mut self) {
        self.rx.close();
        let h = self
            .handle
            .take()
            .expect("You cannont close the data channel twice");
        h.await.unwrap();
    }

    pub async fn next(&mut self) -> Option<(RsaPublicKey, RsaPrivateKey)> {
        if self.sent > 20 {
            None
        } else {
            self.sent += 1;
            self.rx.recv().await
        }
    }
}

// impl Drop for Data { fn drop(&mut self) { self.close(); } }

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
