#![allow(unused)]
#![feature(get_many_mut)]

pub mod data;
pub mod error;
pub mod network;
pub mod prelude;
pub mod trainer;
pub mod traits;
pub mod util;

use crate::prelude::*;
use std::fs;

use crate::{data::Data, network::Network, trainer::Trainer};

const TRAINING_SET: usize = 10;

#[tokio::main]
async fn main() {
    start_logger().await;
    log::debug!("Log init");

    let mut trainer = Trainer::new(make_network().await);

    let mut data = Data::new().await;

    /* while*/
    if let Some((publ_key, priv_key)) = data.next().await {
        // log::info!("Starting new keyset: {:?}", keyset);
        trainer.train(publ_key, priv_key);
    }

    let network = trainer.into_inner();
    let f = fs::File::create("nn.json").unwrap();
    json::to_writer(f, &network);
}

async fn make_network() -> Network {
    match fs::File::open("nn.json")
        .map_err(Into::<Error>::into)
        .and_then(|f| json::from_reader(f).map_err(Into::<Error>::into))
    {
        Ok(n) => n,
        Err(e) => {
            log::error!("Network fetch failure: {}", e);
            log::info!("Creating bare network to remedy");
            Network::new([7, 5, 5, 3])
        }
    }
}

async fn start_logger() {
    simplelog::TermLogger::init(
        log::LevelFilter::Trace,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    );
}

// let mut iter = output.into_iter();
// let n = iter.next().unwrap();
// let e = iter.next().unwrap();
// let d = iter.next().unwrap();
// let primes = iter.filter(|i| *i < BigUint::new(vec![3])).collect();
// rsa::RsaPrivateKey::from_components(n, e, d, primes).unwrap()
