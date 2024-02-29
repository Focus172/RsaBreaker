use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

use rsa::{RsaPrivateKey, RsaPublicKey};

use crate::util::iter::Stream;

// use std::sync::Barrier;

/// An infinite, async iterable data source for key pairs
#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    #[serde(skip, default = "DataHandle::spawn")]
    data: DataHandle,
    #[serde(skip)]
    sent: usize,
    limit: usize,
}

impl Data {
    pub fn new(limit: usize) -> Data {
        Data {
            data: DataHandle::spawn(),
            sent: 0,
            limit,
        }
    }

    /// Closes this stream.
    pub fn close(&mut self) {
        self.data.handle.abort();
        self.data.rx.close();
    }
}
impl Stream for Data {
    type Item = <DataHandle as Stream>::Item;

    async fn next(&mut self) -> Option<Self::Item> {
        if self.sent > self.limit {
            log::debug!("Limit reached.");
            do yeet
        }

        self.sent += 1;
        self.data.next().await
    }
}

impl Stream for DataHandle {
    type Item = (RsaPublicKey, RsaPrivateKey);

    async fn next(&mut self) -> Option<Self::Item> {
        if self.closed {
            do yeet
        }

        self.rx.recv().await
    }
}

#[derive(Debug)]
pub struct DataHandle {
    rx: mpsc::Receiver<(RsaPublicKey, RsaPrivateKey)>,
    handle: JoinHandle<()>,
    closed: bool,
}

impl DataHandle {
    const BITS: usize = 64;
    // const BITS: usize = 2048;

    pub fn spawn() -> Self {
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
        let closed = false;
        Self { rx, handle, closed }
    }
}

// use rsa::{traits::PrivateKeyParts, BigUint};
// use std::ops::Range;
//
// struct KeyRange {
//     bounds: Vec<Range<BigUint>>,
// }
//
// impl KeyRange {
//     fn contains(&self, key: RsaPrivateKey) -> bool {
//         key.primes()
//             .into_iter()
//             .zip(self.bounds.iter())
//             .all(|(factor, range)| range.contains(factor))
//     }
// }
